#![allow(dead_code)]
fn add(a:i64,b:i64)->i64 { a.checked_add(b).expect("overflow") }
fn mul(a:i64,b:i64)->i64 { a.checked_mul(b).expect("overflow") }
fn sub(a:i64,b:i64)->i64 { a.checked_sub(b).expect("overflow") }
fn fib(n:i64)->i64 {if n<2 {n} else {add(fib(sub(n,1)),fib(sub(n,2)))}}
fn main(){
println!("{}",fib(28));
}
