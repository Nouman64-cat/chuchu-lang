#![allow(dead_code)]
fn add(a:i64,b:i64)->i64 { a.checked_add(b).expect("overflow") }
fn mul(a:i64,b:i64)->i64 { a.checked_mul(b).expect("overflow") }
fn sub(a:i64,b:i64)->i64 { a.checked_sub(b).expect("overflow") }

fn main(){
let mut a:[i64;64]=[1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58,59,60,61,62,63,64];let mut total:i64=0;let mut i:i64=0;while i<30000{let mut sum:i64=0;for value in a{sum=add(sum,value);}total=add(total,sum);for j in 0..64{a[j]=add(a[j],1);}i=add(i,1);}println!("{}",total);
}
