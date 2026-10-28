#!/usr/bin/env python3
"""Noise-aware baseline comparison. Reporting by default; optional dedicated-host gate."""
import argparse
import json
from pathlib import Path


def compare(baseline, current, threshold=10.0, noise_limit=0.15):
    old={(row['workload'],row['variant']):row for row in baseline['results']}
    same_environment=all(baseline['environment'].get(key)==current['environment'].get(key) for key in ['platform','machine','cpu','rust','c','python','llvm'])
    same_inputs=baseline.get('source_hashes')==current.get('source_hashes')
    same_policy=all(baseline['config'].get(key)==current['config'].get(key) for key in ['cpu','rust_flags','c_flags'])
    results=[]
    for row in current['results']:
        key=(row['workload'],row['variant']);before=old.get(key)
        record=dict(workload=key[0],variant=key[1])
        if not before or not same_environment or not same_inputs or not same_policy:
            record['status']='incomparable or missing baseline';results.append(record);continue
        a=before['execution'];b=row['execution'];delta=(b['median_s']/a['median_s']-1)*100
        noisy=max(a['iqr_s']/a['median_s'],b['iqr_s']/b['median_s'])>noise_limit
        if noisy:
            status='unreliable/noisy'
        elif delta>threshold and b['q1_s']>a['q3_s']:
            status='possible regression'
        elif delta < -threshold and b['q3_s']<a['q1_s']:
            status='faster'
        else:
            status='unchanged/overlapping intervals'
        record.update(status=status,baseline_median_s=a['median_s'],current_median_s=b['median_s'],percent_change=delta,baseline_iqr_s=a['iqr_s'],current_iqr_s=b['iqr_s'])
        results.append(record)
    return dict(threshold_percent=threshold,noise_limit=noise_limit,results=results)


def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('baseline',type=Path);parser.add_argument('current',type=Path)
    parser.add_argument('--threshold',type=float,default=10.0);parser.add_argument('--noise-limit',type=float,default=0.15)
    parser.add_argument('--output',type=Path);parser.add_argument('--fail-on-regression',action='store_true')
    args=parser.parse_args()
    if args.threshold<0 or args.noise_limit<0:parser.error('Thresholds must be nonnegative')
    result=compare(json.loads(args.baseline.read_text()),json.loads(args.current.read_text()),args.threshold,args.noise_limit)
    text=json.dumps(result,indent=2)+'\n'
    if args.output:args.output.write_text(text)
    else:print(text,end='')
    return int(args.fail_on_regression and any(row['status']=='possible regression' for row in result['results']))

if __name__=='__main__':
    raise SystemExit(main())
