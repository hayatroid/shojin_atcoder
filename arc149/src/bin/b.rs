use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
    }
    let mut ab = a.into_iter().zip(b).collect::<Vec<(_, _)>>();
    ab.sort();
    let mut dp = vec![!0; n];
    for (_, b) in ab {
        let pos = dp.lower_bound(&b);
        dp[pos] = b;
    }
    let lis = dp.lower_bound(&(!0));
    println!("{}", n + lis);
}
