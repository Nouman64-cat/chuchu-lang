#![allow(dead_code)]
fn add(a:i64,b:i64)->i64 { a.checked_add(b).expect("overflow") }
fn mul(a:i64,b:i64)->i64 { a.checked_mul(b).expect("overflow") }
fn sub(a:i64,b:i64)->i64 { a.checked_sub(b).expect("overflow") }

fn main(){
let mut i:i64=0;let mut x:f64=1.0;while i<1000000{x=x*1.0000001+0.0000001;i=add(i,1);}println!("{:.14e}",x);
}
