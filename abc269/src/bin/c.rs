use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: u64,
    }
    let mut ans = vec![n];
    let mut i = n;
    while i > 0 {
        i = (i - 1) & n;
        ans.push(i);
    }
    println!("{}", ans.iter().rev().join("\n"));
}
