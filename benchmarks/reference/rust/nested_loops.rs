#![allow(dead_code)]
fn add(a:i64,b:i64)->i64 { a.checked_add(b).expect("overflow") }
fn mul(a:i64,b:i64)->i64 { a.checked_mul(b).expect("overflow") }
fn sub(a:i64,b:i64)->i64 { a.checked_sub(b).expect("overflow") }

fn main(){
let mut i:i64=0;let mut total:i64=0;while i<500{let mut j:i64=0;while j<500{total=add(total,mul(i,j));j=add(j,1);}i=add(i,1);}println!("{}",total);
}
