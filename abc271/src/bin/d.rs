use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        s: i32,
        ab: [(i32, i32); n],
    }
    let mut dp = vec![HashSet::new()];
    dp[0].insert(0);
    for i in 0..n {
        let (a, b) = ab[i];
        let mut nx = HashSet::new();
        for &x in dp[i].iter() {
            if x + a <= s {
                nx.insert(x + a);
            }
            if x + b <= s {
                nx.insert(x + b);
            }
        }
        dp.push(nx);
    }
    if !dp[n].contains(&s) {
        println!("No");
        return;
    }
    let mut ans = String::new();
    let mut pos = s;
    for i in (0..n).rev() {
        let (a, b) = ab[i];
        if dp[i].contains(&(pos - a)) {
            ans = format!("H{}", ans);
            pos -= a;
        } else {
            ans = format!("T{}", ans);
            pos -= b;
        }
    }
    println!("Yes");
    println!("{}", ans);
}
