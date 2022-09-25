# トヨタ自動車プログラミングコンテスト2022 (AtCoder Beginner Contest 270)


## A - 1-2-4 Test
- ビット演算 (OR) を用いて簡潔にする

## B - Hammer
- 場合分けの数を減らす
- abs, max, min の活用
- 原点に関する対称性より，ゴール・壁・ハンマーの符号を反転してもよい
- 壁とゴール，壁とハンマーの位置を比較するので壁の符号を正に決めておくとやりやすい

## C - Simple path
- DFS / BFS の利用
- 親を記録しておき，ゴールからたどっていく

### DFS
```rust
let mut dfs = vec![s];
while let Some(v) = dfs.pop() {
    visited[v] = true;
    // 現在の頂点に対する操作
    for &u in &g[v] {
        if visited[u] { continue }
        // 次の頂点に対する操作
    }
}
```
### BFS
```rust
let mut bfs = VecDeque::new();
bfs.push_back(s)
while let Some(v) = bfs.pop_front() {
    visited[v] = true;
    // 現在の頂点に対する操作
    for &u in &g[v] {
        if visited[u] { continue }
        // 次の頂点に対する操作
    }
}