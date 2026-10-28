#!/usr/bin/env python3
"""Correctness-first, process-level benchmarks. Python standard library only."""
import argparse
import datetime
from decimal import Decimal
import hashlib
import json
import os
from pathlib import Path
import platform
import random
import re
import shutil
import statistics
import subprocess
import sys
import time

ROOT = Path(__file__).resolve().parent
REPO = ROOT.parent


def command(args):
    return subprocess.run([str(x) for x in args], capture_output=True, text=True, check=True)


def summary(samples):
    q1, _, q3 = statistics.quantiles(samples, n=4, method='inclusive') if len(samples) > 1 else (samples[0], samples[0], samples[0])
    return dict(samples_s=samples, median_s=statistics.median(samples), q1_s=q1, q3_s=q3, iqr_s=q3-q1)


def version(args):
    try:
        result = command(args)
        return (result.stdout or result.stderr).strip()
    except (OSError, subprocess.CalledProcessError) as error:
        return str(error)


def valid_output(output, workload):
    if output.returncode != 0 or output.stderr:
        raise RuntimeError(f"{workload['name']}: exit={output.returncode}, stderr={output.stderr}")
    actual = output.stdout.strip()
    wanted = workload['expected']
    if workload['numeric_output']:
        # References may use scientific notation. Compare the identical 15-digit value,
        # not a broad floating tolerance. All outputs must contain exactly one number.
        correct = len(actual.split()) == 1 and Decimal(actual) == Decimal(wanted)
    else:
        correct = output.stdout == wanted + '\n'
    if not correct:
        raise RuntimeError(f"{workload['name']}: checksum {actual!r} != {wanted!r}")


def memory_run(args, validator=None):
    if sys.platform != 'darwin' or not Path('/usr/bin/time').exists():
        return None
    result = command(['/usr/bin/time', '-l', *args])
    match = re.search(r'(\d+)\s+maximum resident set size', result.stderr)
    if validator:
        # Only /usr/bin/time's resource report is removed; the prior untimed/timed
        # executions independently require clean stderr and a valid result.
        validator(result.stdout)
    return int(match.group(1)) if match else None


def has_cycle(graph, root):
    active, seen = set(), set()
    def visit(node):
        if node in active:
            return True
        if node in seen:
            return False
        seen.add(node); active.add(node)
        found = any(visit(child) for child in graph.get(node, []) if child in graph)
        active.remove(node)
        return found
    return visit(root)


def inspect_ir(ir_path, assembly_path):
    ir = ir_path.read_text()
    assembly = assembly_path.read_text()
    bodies = dict(re.findall(r'^define\b[^@]*@([\w.]+)\([^\n]*\)[^{]*\{(.*?)^}', ir, re.M | re.S))
    calls = {name: re.findall(r'\bcall\b[^\n]*@([\w.]+)\(', body) for name, body in bodies.items()}
    reachable = set()
    def reach(name):
        if name in reachable or name not in bodies:
            return
        reachable.add(name)
        for callee in calls[name]:
            reach(callee)
    reach('main')
    loop_functions = []
    for name in reachable:
        body = bodies[name]
        chunks = re.split(r'^([\w.]+):[^\n]*\n', body, flags=re.M)
        graph = {chunks[i]: re.findall(r'label %([\w.]+)', chunks[i+1]) for i in range(1, len(chunks), 2)}
        if graph and has_cycle(graph, next(iter(graph))):
            loop_functions.append(name)
    recursion = has_cycle({name: [x for x in calls[name] if x in reachable] for name in reachable}, 'main')
    vector_ir = re.findall(r'\b(?:fadd|fmul|fsub|fdiv|add|mul)\s+<\d+ x [^>]+>', ir)
    vector_asm = re.findall(r'\b(?:fadd|fmul|fsub|fdiv|fmla)(?:\.\d+[ds]\s+v\d+|\s+v\d+\.\d+[ds])', assembly)
    return dict(allocas=len(re.findall(r'\balloca\b', ir)), loads=len(re.findall(r'\bload\b', ir)), stores=len(re.findall(r'\bstore\b', ir)),
                llvm_lines=len(ir.splitlines()), vector_arithmetic_ir=len(vector_ir), vector_arithmetic_assembly=len(vector_asm),
                reachable_loop_functions=sorted(loop_functions), reachable_recursion=recursion,
                classification='runtime loops/recursion retained; static inputs' if loop_functions or recursion else 'no reachable loop/recursion: potentially compile-time folded; not runtime throughput evidence')


def stage_json(stderr):
    return json.loads(next(line for line in reversed(stderr.splitlines()) if line.startswith('{')))


def report(data, path):
    rows = data['results']
    labels = ['chuchu-O0', 'chuchu-O1', 'chuchu-O2', 'chuchu-O3', 'rust', 'c', 'python']
    text = '# Benchmark results\n\nExecution is wall time per process (ms), including startup; compilation excluded.\n\n'
    text += '| Workload | ' + ' | '.join(labels) + ' |\n|---|' + '---:|'*len(labels) + '\n'
    for workload in data['workloads']:
        values = {row['variant']: row for row in rows if row['workload'] == workload}
        text += '| ' + workload + ' | ' + ' | '.join(f"{values[label]['execution']['median_s']*1000:.3f}" if label in values else 'N/A' for label in labels) + ' |\n'
    text += '\n## Individual measurements\n\n| Workload / variant | Median ms | IQR ms | Compile ms | Bytes | Peak RSS bytes |\n|---|---:|---:|---:|---:|---:|\n'
    for row in rows:
        text += f"| {row['workload']} / {row['variant']} | {row['execution']['median_s']*1000:.3f} | {row['execution']['iqr_s']*1000:.3f} | {format(row['compilation']['median_s']*1000, '.3f') if row['compilation'] else 'N/A'} | {row['executable_bytes'] if row['executable_bytes'] is not None else 'N/A'} | {row['peak_rss_bytes']} |\n"
    text += '\n## Controlled optimization comparison\n\nRatios below are O0 median / optimized median (>1 favors optimized), not isolated kernel throughput.\n\n| Workload | O0/O1 | O0/O2 | O0/O3 |\n|---|---:|---:|---:|\n'
    for workload in data['workloads']:
        values = {row['variant']: row for row in rows if row['workload'] == workload}
        if 'chuchu-O0' in values:
            baseline = values['chuchu-O0']['execution']['median_s']
            text += '| '+workload+' | '+' | '.join(f"{baseline/values[label]['execution']['median_s']:.2f}x" if label in values else 'N/A' for label in labels[1:4])+' |\n'
    text += '\n## IR inspection\n\n'
    for row in rows:
        if 'inspection' in row:
            detail=row['inspection']
            text += f"- {row['workload']} / {row['variant']}: {detail['classification']}; allocas={detail['allocas']}, loads={detail['loads']}, stores={detail['stores']}, vector arithmetic IR/assembly={detail['vector_arithmetic_ir']}/{detail['vector_arithmetic_assembly']}.\n"
    text += '\nAll raw samples, compilation stage timings, machine/toolchain metadata, startup controls, source hashes, memory measurements, and compiler-throughput samples are in the adjacent JSON. Static inputs, process startup, bounds/overflow semantics, and Python allocation/interpreter overhead limit cross-language conclusions. Memory is measured separately with macOS time -l; Python executable size is not comparable and is N/A.\n'
    path.write_text(text)


def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--cheeku',type=Path,default=REPO/'target/release/cheeku')
    parser.add_argument('--output',type=Path,default=ROOT/'results/latest.json')
    parser.add_argument('--artifacts',type=Path)
    parser.add_argument('--repetitions',type=int,default=7)
    parser.add_argument('--compile-repetitions',type=int,default=3)
    parser.add_argument('--warmups',type=int,default=1)
    parser.add_argument('--workloads',default='')
    parser.add_argument('--levels',default='O0,O1,O2,O3')
    parser.add_argument('--languages',default='chuchu,rust,c,python')
    parser.add_argument('--baseline',type=Path)
    parser.add_argument('--regression-threshold',type=float,default=10.0)
    parser.add_argument('--no-simd-probe',action='store_true')
    parser.add_argument('--no-throughput',action='store_true')
    args=parser.parse_args()
    if args.repetitions<3 or args.compile_repetitions<1 or args.warmups<0:
        parser.error('Require repetitions >=3, compile-repetitions >=1, warmups >=0')
    levels=args.levels.split(','); languages=args.languages.split(',')
    if not set(levels)<=set(['O0','O1','O2','O3']) or not set(languages)<=set(['chuchu','rust','c','python']):
        parser.error('Unsupported optimization level or reference language')
    manifest=json.loads((ROOT/'manifest.json').read_text())
    if args.workloads:
        requested=set(args.workloads.split(','))
        if not requested <= {workload['name'] for workload in manifest}: parser.error('Unknown workload')
        manifest=[workload for workload in manifest if workload['name'] in requested]
    artifact=(args.artifacts or REPO/'target/cheeku-bench'/str(time.time_ns())).resolve();artifact.mkdir(parents=True,exist_ok=False)
    cheeku=args.cheeku.resolve()
    tools={name:shutil.which(name) for name in ['rustc','clang']}
    source_hashes={str(path.relative_to(ROOT)):hashlib.sha256(path.read_bytes()).hexdigest() for directory in ['chuchu','reference'] for path in (ROOT/directory).rglob('*') if path.is_file()}
    environment=dict(cpu_count=os.cpu_count(), ram_bytes=version(['sysctl','-n','hw.memsize']),platform=platform.platform(),machine=platform.machine(),cpu=version(['sysctl','-n','machdep.cpu.brand_string']),os=version(['sw_vers']),rust=version(['rustc','--version']),c=version(['clang','--version']),python=sys.version,llvm=version([str(Path(version(['brew','--prefix','llvm']))/'bin/llvm-config'),'--version']))
    data=dict(schema=1,created_utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),environment=environment,git_revision=version(['git','rev-parse','HEAD']),compiler_sha256=hashlib.sha256(cheeku.read_bytes()).hexdigest(),source_hashes=source_hashes,
              config=dict(repetitions=args.repetitions,compile_repetitions=args.compile_repetitions,warmups=args.warmups,cpu='generic',rust_flags='-O -C target-cpu=generic',c_flags='-O2 -ffp-contract=off',timing='wall process time including startup; compilation excluded from execution',memory='one separate macOS /usr/bin/time -l run, bytes'),workloads=[w['name'] for w in manifest],results=[],skipped=[],compiler_throughput=[])
    plans=[]
    for workload in manifest:
        name=workload['name']; print(f'Building {name}',flush=True)
        variants=[('chuchu-'+level,'chuchu',level,True) for level in levels] if 'chuchu' in languages else []
        variants += [(language,language,None,True) for language in languages if language!='chuchu']
        if not args.no_simd_probe and name=='array_multiply' and 'chuchu' in languages and 'O3' in levels:
            variants.append(('chuchu-O3-no-vectorize','chuchu','O3',False))
        for label,language,level,vectorize in variants:
            exe=artifact/(name+'-'+label); stages=[];compile_samples=[]
            if language in ('rust','c') and not tools['rustc' if language=='rust' else 'clang']:
                data['skipped'].append(dict(workload=name,variant=label,reason='Tool unavailable'));continue
            if language=='chuchu':
                source=ROOT/'chuchu'/(name+'.chuchu')
                compile_cmd=[str(cheeku),str(source),'-'+level,'--time-compilation','-o',str(exe)]
                if not vectorize:compile_cmd.append('--no-vectorize')
                run_cmd=[str(exe)]
            elif language=='rust':
                compile_cmd=[tools['rustc'],'-O','-C','target-cpu=generic',str(ROOT/'reference/rust'/(name+'.rs')),'-o',str(exe)];run_cmd=[str(exe)]
            elif language=='c':
                compile_cmd=[tools['clang'],'-O2','-ffp-contract=off',str(ROOT/'reference/c'/(name+'.c')),'-o',str(exe)];run_cmd=[str(exe)]
            else:
                compile_cmd=None;run_cmd=[sys.executable,str(ROOT/'reference/python'/(name+'.py'))]
            if compile_cmd:
                for _ in range(args.compile_repetitions):
                    start=time.perf_counter_ns();result=command(compile_cmd);compile_samples.append((time.perf_counter_ns()-start)/1e9)
                    if language=='chuchu':stages.append(stage_json(result.stderr))
            else:compile_samples=[]
            valid_output(command(run_cmd),workload)
            row=dict(workload=name,variant=label,parameters=workload['parameters'],compilation=summary(compile_samples) if compile_samples else None,compilation_stages=stages,executable_bytes=exe.stat().st_size if compile_cmd else None,peak_rss_bytes=memory_run(run_cmd),compile_peak_rss_bytes=memory_run(compile_cmd) if compile_cmd else None,execution_samples=[])
            if language=='chuchu':
                ir=artifact/(name+'-'+label+'.ll');asm=artifact/(name+'-'+label+'.s')
                # Build diagnostics into a separate executable, never the timed binary.
                inspect_cmd=compile_cmd.copy();index=inspect_cmd.index('-o');inspect_cmd[index+1]=str(exe)+'-inspect'
                command(inspect_cmd+['--emit-ir',str(ir),'--emit-assembly',str(asm)])
                row['inspection']=inspect_ir(ir,asm);row['ir_path']=os.path.relpath(ir, REPO);row['assembly_path']=os.path.relpath(asm, REPO)
            data['results'].append(row);plans.append((row,run_cmd,workload))
    for _,run_cmd,workload in plans:
        for _ in range(args.warmups):valid_output(command(run_cmd),workload)
    rng=random.Random(7)
    for sample in range(args.repetitions):
        rng.shuffle(plans);print(f'Execution round {sample+1}/{args.repetitions}',flush=True)
        for row,run_cmd,workload in plans:
            start=time.perf_counter_ns();result=command(run_cmd);elapsed=(time.perf_counter_ns()-start)/1e9
            valid_output(result,workload);row['execution_samples'].append(elapsed)
    for row in data['results']:row['execution']=summary(row.pop('execution_samples'))
    data['startup_controls']={}
    for label,run_cmd in [('native-empty',['/usr/bin/true']),('python-empty',[sys.executable,'-c','pass'])]:
        samples=[]
        for _ in range(args.repetitions):
            start=time.perf_counter_ns();command(run_cmd);samples.append((time.perf_counter_ns()-start)/1e9)
        data['startup_controls'][label]=summary(samples)
    if not args.no_throughput and 'chuchu' in languages:
        for count in [32,256,1024]:
            source=artifact/f'throughput-{count}.chuchu';source.write_text('\n'.join(f'set value{i} to {i}' for i in range(count))+f'\nshow value{count-1}\n')
            for level in levels:
                stages=[];samples=[];exe=artifact/f'throughput-{count}-{level}'
                for _ in range(args.compile_repetitions):
                    start=time.perf_counter_ns();result=command([cheeku,source,'-'+level,'--time-compilation','-o',exe]);samples.append((time.perf_counter_ns()-start)/1e9);stages.append(stage_json(result.stderr))
                assert command([exe]).stdout==str(count-1)+'\n'
                data['compiler_throughput'].append(dict(statements=count+1,source_bytes=source.stat().st_size,variant='chuchu-'+level,compilation=summary(samples),stages=stages))
    args.output.parent.mkdir(parents=True,exist_ok=True)
    args.output.write_text(json.dumps(data,indent=2)+'\n');report(data,args.output.with_suffix('.md'))
    if args.baseline:
        command([sys.executable,ROOT/'compare.py',args.baseline,args.output,'--threshold',str(args.regression_threshold),'--output',args.output.with_suffix('.regressions.json')])
    print(f'Results: {args.output}',flush=True)

if __name__=='__main__':
    main()
