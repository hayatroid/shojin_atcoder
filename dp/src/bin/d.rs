use proconio::input;

fn main() {
    input! {
        n: usize,
        w_lim: usize,
        wv: [(usize, u64); n],
    }
    let mut dp = vec![0; w_lim + 1];
    for (w, v) in wv {
        for i in (w..=w_lim).rev() {
            dp[i] = dp[i].max(dp[i - w] + v);
        }
    }
    println!("{}", dp[w_lim]);
}
