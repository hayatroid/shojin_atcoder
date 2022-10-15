use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        a: [u32; n],
    }
    let mut b = a.clone();
    b.sort();
    b.dedup();
    let mut ans = vec![0; n];
    for a in a {
        ans[b.len() - b.upper_bound(&a)] += 1;
    }
    for ans in ans {
        println!("{}", ans);
    }
}
