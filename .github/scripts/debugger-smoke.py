#!/usr/bin/env python3
"""Exercise real LLDB source stops and values after compiler staging cleanup.

Requires LLDB with Python scripting. Missing tools are failures, not silent skips.
"""
import argparse
import os
from pathlib import Path
import shutil
import signal
import subprocess
import tempfile


SOURCE = """function bump(n: int) -> int {
    assign next as n + 1
    return next
}
assign counter as 0
assign fraction as 1.5
assign enabled as true
assign values as [2, 4, 6]
while counter < 2 {
    reassign counter as bump(counter)
}
if enabled {
    assign counter as 99
    show counter
}
show counter
show fraction
show values[1]
show enabled
assign greeting as "hello"
reassign greeting as greeting + " strings"
show greeting
assign parsed as parse_int("42")
assign line as ok("result text")
show unwrap(parsed)
show unwrap(line)
"""


def quoted(value):
    # LLDB command-file quoting, not shell quoting.
    return '"' + str(value).replace('\\', '\\\\').replace('"', '\\"') + '"'


def run_debugger(command, cwd):
    process = subprocess.Popen(command, cwd=cwd, stdout=subprocess.PIPE,
                               stderr=subprocess.STDOUT, text=True,
                               start_new_session=os.name != 'nt')
    try:
        output, _ = process.communicate(timeout=90)
    except subprocess.TimeoutExpired:
        if os.name == 'nt':
            subprocess.run(['taskkill', '/F', '/T', '/PID', str(process.pid)], check=False)
        else:
            os.killpg(process.pid, signal.SIGKILL)
        output, _ = process.communicate()
        raise RuntimeError('LLDB timed out:\n' + output)
    print(output)
    if process.returncode:
        raise RuntimeError(f'LLDB exited {process.returncode}')
    return output


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--compiler', required=True)
    parser.add_argument('--lldb', default=os.environ.get('CHEEKU_LLDB', 'lldb'))
    args = parser.parse_args()
    compiler = Path(args.compiler).resolve(strict=True)
    debugger = shutil.which(args.lldb)
    if not debugger:
        raise RuntimeError('LLDB is required; supply --lldb or CHEEKU_LLDB')
    subprocess.run([debugger, '--version'], check=True)
    with tempfile.TemporaryDirectory(prefix='cheeku debugger ') as temporary:
        root = Path(temporary).resolve()
        source = root / 'source with spaces.chuchu'
        source.write_text(SOURCE, encoding='utf-8')
        executable = root / ('program.exe' if os.name == 'nt' else 'program')
        subprocess.run([str(compiler), str(source), '--debug', '-o', str(executable)],
                       check=True, timeout=60)
        assert not list(root.glob('.cheeku-*')), 'Compiler staging survived'
        # Move both final artifacts before debugging: object and staging paths
        # must not be necessary for symbol discovery.
        relocated = root / 'relocated'
        relocated.mkdir()
        for artifact in list(root.iterdir()):
            if artifact.name.startswith(executable.name):
                artifact.rename(relocated / artifact.name)
        executable = relocated / executable.name
        checks = []
        commands = ['settings set auto-confirm true',
                    'breakpoint set --file ' + quoted(source) + ' --line 9', 'run']

        def check(name, expression):
            checks.append(name)
            commands.append(f"script print('CHECK {name}', bool({expression}))")

        check('initial-line', 'lldb.frame.GetLineEntry().GetLine() == 9')
        check('initial-counter', 'lldb.frame.FindVariable("counter").IsValid() and lldb.frame.FindVariable("counter").GetValueAsSigned() == 0')
        check('float', 'float(lldb.frame.FindVariable("fraction").GetValue()) == 1.5')
        check('bool', 'lldb.frame.FindVariable("enabled").GetValueAsUnsigned() == 1')
        check('array', 'lldb.frame.FindVariable("values").GetChildAtIndex(1).GetValueAsSigned() == 4')
        commands += ['thread step-over', 'thread step-in']
        check('step-in', 'lldb.frame.GetLineEntry().GetLine() == 2')
        check('parameter', 'lldb.frame.FindVariable("n").GetValueAsSigned() == 0')
        check('stack', 'lldb.thread.GetNumFrames() >= 2')
        commands += ['thread step-over']
        check('local', 'lldb.frame.FindVariable("next").GetValueAsSigned() == 1')
        commands += ['thread step-out', 'continue']
        check('loop-reassignment', 'lldb.frame.FindVariable("counter").GetValueAsSigned() == 1')
        commands += ['breakpoint disable 1',
                     'breakpoint set --file ' + quoted(source) + ' --line 14',
                     'breakpoint set --file ' + quoted(source) + ' --line 16', 'continue']
        check('inner-line', 'lldb.frame.GetLineEntry().GetLine() == 14')
        check('shadow', 'lldb.frame.FindVariable("counter").GetValueAsSigned() == 99')
        commands += ['continue']
        check('outer-line', 'lldb.frame.GetLineEntry().GetLine() == 16')
        check('outer-scope', 'lldb.frame.FindVariable("counter").GetValueAsSigned() == 2')
        commands += ['breakpoint set --file ' + quoted(source) + ' --line 22', 'continue']
        check('string-line', 'lldb.frame.GetLineEntry().GetLine() == 22')
        check('string-value', '"hello strings" in (lldb.frame.FindVariable("greeting").GetSummary() or "")')
        commands += ['breakpoint set --file ' + quoted(source) + ' --line 25', 'continue']
        check('result-line', 'lldb.frame.GetLineEntry().GetLine() == 25')
        check('result-code', 'lldb.frame.FindVariable("parsed").GetChildMemberWithName("code").GetValueAsSigned() == 0')
        check('result-int', 'lldb.frame.FindVariable("parsed").GetChildMemberWithName("value").GetValueAsSigned() == 42')
        check('result-string', '"result text" in (lldb.frame.FindVariable("line").GetChildMemberWithName("value").GetSummary() or "")')
        commands += ['continue']
        check('exit', 'lldb.process.GetExitStatus() == 0')
        script = root / 'commands.lldb'
        script.write_text('\n'.join(commands) + '\n', encoding='utf-8')
        output = run_debugger([debugger, '--batch', '--source', str(script), str(executable)], root)
        for name in checks:
            assert f'CHECK {name} True' in output, f'Failed debugger assertion: {name}'
        print(f'Passed {len(checks)} debugger assertions after cleanup and relocation.')


if __name__ == '__main__':
    main()
