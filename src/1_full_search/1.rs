// 整数N,VとN個の整数A0〜A(N-1)が与えられる。N個の整数の中にVがあるか判定せよ。
use std::io;

fn main() {
  let mut N:String;
  let mut V:String;
  io::stdin().read_line(&mut N);
  let n:i32 = N.parse();
  io::stdin().read_line(&mut V);
  let v:i32 = V.parse();
  let A: [i32; n];
  for i in 0..n {
    io::stdin().read_line(&mut A[i]);
  }

  //線形探索
  let mut flag = false;
  for i in 0..n {
    if A[i] == v {flag = true; break}
  }
  if flag == true {
    println!("Yes");
  } else {
    println!("No");
  }
}