use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut num = 0;
    let mut rep = 0;
    for i in 1..=9 {
        let mut tmp = 0;
        for j in 1..=n {
            tmp = (tmp * 10 + i) % m;
            if tmp == 0 && j >= rep {
                num = i;
                rep = j;
            }
        }
    }
    if rep > 0 {
        println!("{}", vec![num; rep].iter().join(""))
    } else {
        println!("-1")
    }
}
