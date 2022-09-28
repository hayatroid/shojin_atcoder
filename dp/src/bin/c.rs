use proconio::input;

fn main() {
    input! {
        n: usize,
        abc: [(u32, u32, u32); n],
    }
    let mut dp = vec![vec![0; 3]; n + 1];
    for (i, &(a, b, c)) in abc.iter().enumerate() {
        dp[i + 1][0] = dp[i][1].max(dp[i][2]) + a;
        dp[i + 1][1] = dp[i][2].max(dp[i][0]) + b;
        dp[i + 1][2] = dp[i][0].max(dp[i][1]) + c;
    }
    println!("{}", dp[n].iter().max().unwrap());
}
