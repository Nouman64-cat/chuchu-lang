#!/usr/bin/env python3
"""Generate fixed, equivalent, valid-input workloads; no language extensions or dynamic input."""
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parent
manifest = []


def emit(name, chuchu, c, rust, python, expected, parameters, floating=False, extra_c='', extra_rust='', extra_python=''):
    (ROOT / 'chuchu' / (name + '.chuchu')).write_text(chuchu.strip() + '\n')
    c_head = '''#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
static inline int64_t add(int64_t a,int64_t b){int64_t r;if(__builtin_add_overflow(a,b,&r))exit(1);return r;}
static inline int64_t mul(int64_t a,int64_t b){int64_t r;if(__builtin_mul_overflow(a,b,&r))exit(1);return r;}
static inline int64_t sub(int64_t a,int64_t b){int64_t r;if(__builtin_sub_overflow(a,b,&r))exit(1);return r;}
'''
    r_head = '''#![allow(dead_code)]
fn add(a:i64,b:i64)->i64 { a.checked_add(b).expect("overflow") }
fn mul(a:i64,b:i64)->i64 { a.checked_mul(b).expect("overflow") }
fn sub(a:i64,b:i64)->i64 { a.checked_sub(b).expect("overflow") }
'''
    (ROOT / 'reference/c' / (name + '.c')).write_text(c_head + extra_c + '\nint main(void){\n' + c + '\nreturn 0;\n}\n')
    (ROOT / 'reference/rust' / (name + '.rs')).write_text(r_head + extra_rust + '\nfn main(){\n' + rust + '\n}\n')
    (ROOT / 'reference/python' / (name + '.py')).write_text(extra_python + '\n' + python.strip() + '\n')
    manifest.append(dict(name=name, parameters=parameters, expected=str(expected), numeric_output=floating))

n=3000000
emit('integer_loop', f'''assign i as 0
assign total as 0
while i < {n} {{
reassign total as total + i
reassign i as i + 1
}}
show total''',
     f'int64_t i=0,total=0;while(i<{n}){{total=add(total,i);i=add(i,1);}}printf("%lld\\n",(long long)total);',
     f'let mut i:i64=0;let mut total:i64=0;while i<{n}{{total=add(total,i);i=add(i,1);}}println!("{{}}",total);',
     f'i=0\ntotal=0\nwhile i<{n}:\n    total+=i\n    i+=1\nprint(total)', n*(n-1)//2, {'iterations':n})

n=1000000
x=1.0
for _ in range(n): x=x*1.0000001+0.0000001
emit('float_loop', f'''assign i as 0
assign x as 1.0
while i < {n} {{
reassign x as x * 1.0000001 + 0.0000001
reassign i as i + 1
}}
show x''',
     f'int64_t i=0;double x=1.0;while(i<{n}){{x=x*1.0000001+0.0000001;i=add(i,1);}}printf("%.15g\\n",x);',
     f'let mut i:i64=0;let mut x:f64=1.0;while i<{n}{{x=x*1.0000001+0.0000001;i=add(i,1);}}println!("{{:.14e}}",x);',
     f'i=0\nx=1.0\nwhile i<{n}:\n    x=x*1.0000001+0.0000001\n    i+=1\nprint(format(x,".15g"))', format(x,'.15g'), {'iterations':n}, True)

n=300000
emit('function_calls', f'''function square_plus(x: int) -> int {{ return x * x + 3 }}
assign i as 0
assign total as 0
while i < {n} {{
reassign total as total + square_plus(i)
reassign i as i + 1
}}
show total''',
     f'int64_t i=0,total=0;while(i<{n}){{total=add(total,square_plus(i));i=add(i,1);}}printf("%lld\\n",(long long)total);',
     f'let mut i:i64=0;let mut total:i64=0;while i<{n}{{total=add(total,square_plus(i));i=add(i,1);}}println!("{{}}",total);',
     f'i=0\ntotal=0\nwhile i<{n}:\n    total+=square_plus(i)\n    i+=1\nprint(total)', (n-1)*n*(2*n-1)//6+3*n, {'calls':n},
     extra_c='static int64_t square_plus(int64_t x){return add(mul(x,x),3);}',
     extra_rust='fn square_plus(x:i64)->i64 {add(mul(x,x),3)}',
     extra_python='def square_plus(x):\n    return x*x+3\n')

n=28
emit('recursion', f'''function fib(n: int) -> int {{
if n < 2 {{ return n }} else {{ return fib(n - 1) + fib(n - 2) }}
}}
show fib({n})''', f'printf("%lld\\n",(long long)fib({n}));', f'println!("{{}}",fib({n}));', f'print(fib({n}))', 317811, {'fibonacci_n':n},
     extra_c='static int64_t fib(int64_t n){if(n<2)return n;return add(fib(sub(n,1)),fib(sub(n,2)));}',
     extra_rust='fn fib(n:i64)->i64 {if n<2 {n} else {add(fib(sub(n,1)),fib(sub(n,2)))}}',
     extra_python='def fib(n):\n    if n<2: return n\n    return fib(n-1)+fib(n-2)\n')

n=500
emit('nested_loops', f'''assign i as 0
assign total as 0
while i < {n} {{
assign j as 0
while j < {n} {{
reassign total as total + i * j
reassign j as j + 1
}}
reassign i as i + 1
}}
show total''',
     f'int64_t i=0,total=0;while(i<{n}){{int64_t j=0;while(j<{n}){{total=add(total,mul(i,j));j=add(j,1);}}i=add(i,1);}}printf("%lld\\n",(long long)total);',
     f'let mut i:i64=0;let mut total:i64=0;while i<{n}{{let mut j:i64=0;while j<{n}{{total=add(total,mul(i,j));j=add(j,1);}}i=add(i,1);}}println!("{{}}",total);',
     f'i=0\ntotal=0\nwhile i<{n}:\n    j=0\n    while j<{n}:\n        total+=i*j\n        j+=1\n    i+=1\nprint(total)', (n*(n-1)//2)**2, {'outer':n,'inner':n})

size=64;n=30000
for name,floating in [('array_multiply',True),('array_reduce',False)]:
    numbers=[f'{i}.0' if floating else str(i) for i in range(1,size+1)]
    ones=['1.0' if floating else '1']*size
    literal='['+','.join(numbers)+']';delta='['+','.join(ones)+']'
    c_type='double' if floating else 'int64_t';rust_type='f64' if floating else 'i64';zero='0.0' if floating else '0'
    product='assign product as a * a\nreassign total as total + sum(product)' if floating else 'reassign total as total + sum(a)'
    source=f'assign a as {literal}\nassign delta as {delta}\nassign i as 0\nassign total as {zero}\nwhile i < {n} {{\n{product}\nreassign a as a + delta\nreassign i as i + 1\n}}\nshow total'
    if floating:
        c_op='double product[64];for(int j=0;j<64;j++)product[j]=a[j]*a[j];double sum=0.0;for(int j=0;j<64;j++)sum+=product[j];total+=sum;'
        r_op='let mut product=[0.0;64];for j in 0..64{product[j]=a[j]*a[j];}let mut sum=0.0;for value in product{sum+=value;}total+=sum;'
        py_op='    product=[x*x for x in a]\n    subtotal=0.0\n    for x in product: subtotal+=x\n    total+=subtotal\n'
        expected=sum(sum((j+t)**2 for j in range(1,size+1)) for t in range(n))
        c_print='printf("%.15g\\n",total);';r_print='println!("{:.14e}",total);';py_print='print(format(total,".15g"))'
    else:
        c_op='int64_t sum=0;for(int j=0;j<64;j++)sum=add(sum,a[j]);total=add(total,sum);'
        r_op='let mut sum:i64=0;for value in a{sum=add(sum,value);}total=add(total,sum);'
        py_op='    subtotal=0\n    for x in a: subtotal+=x\n    total+=subtotal\n'
        expected=n*sum(range(1,size+1))+size*n*(n-1)//2
        c_print='printf("%lld\\n",(long long)total);';r_print='println!("{}",total);';py_print='print(total)'
    increment='a[j]+1.0' if floating else 'add(a[j],1)'
    c=f'{c_type} a[64]={{{",".join(numbers)}}};{c_type} total={zero};int64_t i=0;while(i<{n}){{{c_op}for(int j=0;j<64;j++)a[j]={increment};i=add(i,1);}}{c_print}'
    rust=f'let mut a:[{rust_type};64]=[{",".join(numbers)}];let mut total:{rust_type}={zero};let mut i:i64=0;while i<{n}{{{r_op}for j in 0..64{{a[j]={increment};}}i=add(i,1);}}{r_print}'
    py=f'a={literal}\ni=0\ntotal={zero}\nwhile i<{n}:\n{py_op}    a=[x+1 for x in a]\n    i+=1\n{py_print}'
    emit(name,source,c,rust,py,format(float(expected),'.15g') if floating else expected,{'iterations':n,'elements':size},floating)

size=256;n=30000
a='['+','.join(f'{i}.0' for i in range(1,size+1))+']';b='['+','.join(f'{i}.0' for i in range(2,size+2))+']'
expected=(n//2)*3
emit('array_copies',f'''assign a as {a}
assign b as {b}
assign total as 0.0
assign i as 0
while i < {n} {{
assign temporary as a
reassign a as b
reassign b as temporary
reassign total as total + a[0]
reassign i as i + 1
}}
show total''',
     f'double a[256]={{{a[1:-1]}}},b[256]={{{b[1:-1]}}};double total=0.0;int64_t i=0;while(i<{n}){{double temporary[256];memcpy(temporary,a,sizeof a);memcpy(a,b,sizeof a);memcpy(b,temporary,sizeof a);total+=a[0];i=add(i,1);}}printf("%.15g\\n",total);',
     f'let mut a=[{a[1:-1]}];let mut b=[{b[1:-1]}];let mut total:f64=0.0;let mut i:i64=0;while i<{n}{{let temporary=a;a=b;b=temporary;total+=a[0];i=add(i,1);}}println!("{{:.14e}}",total);',
     f'a={a}\nb={b}\ntotal=0.0\ni=0\nwhile i<{n}:\n    temporary=a.copy()\n    a=b.copy()\n    b=temporary\n    total+=a[0]\n    i+=1\nprint(format(total,".15g"))',expected,{'iterations':n,'elements':size,'array_bytes':size*8},True)
(ROOT/'manifest.json').write_text(json.dumps(manifest,indent=2)+'\n')
