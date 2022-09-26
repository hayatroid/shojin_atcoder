use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        h: [i32; n],
    }
    let mut dp = vec![std::i32::MAX; n];
    dp[0] = 0;
    for i in 1..n {
        for j in 1..=k {
            if i >= j {
                dp[i] = dp[i].min(dp[i - j] + (h[i] - h[i - j]).abs());
            }
        }
    }
    println!("{}", dp[n - 1]);
}
