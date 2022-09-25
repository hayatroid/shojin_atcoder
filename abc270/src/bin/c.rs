use itertools::Itertools;
use proconio::{input, marker::Usize1};
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        x: Usize1,
        y: Usize1,
        uv: [(Usize1, Usize1); n - 1],
    }
    let mut g = vec![vec![]; n];
    for (u, v) in uv {
        g[u].push(v);
        g[v].push(u);
    }
    let mut bfs = VecDeque::new();
    let mut parent = vec![!0; n];
    bfs.push_back(x);
    while let Some(v) = bfs.pop_front() {
        for &u in &g[v] {
            if u == parent[v] {
                continue;
            }
            bfs.push_back(u);
            parent[u] = v;
        }
    }
    let mut pos = y;
    let mut ans = vec![y];
    while pos != x {
        pos = parent[pos];
        ans.push(pos);
    }
    ans.reverse();
    println!("{}", ans.iter().map(|&x| x + 1).join(" "));
}
