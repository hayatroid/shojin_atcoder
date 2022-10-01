use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [[u32]; n],
        st: [(Usize1, Usize1); q],
    }
    for (s, t) in st {
        println!("{}", a[s][t]);
    }
}
