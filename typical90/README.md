# 競プロ典型 90 問


## 001 - Yokan Party（★4）
- 逆像法の利用（長さ x は OK か？）
- 命題「長さ x 以上の，K + 1 個のピースに分割することができる」を P(x) としたとき，P(x) が真となる最大の x を求めればよい
- 二分探索の利用

### 二分探索
```rust
let mut ok = 0;
let mut ng = l;
while ng - ok > 1 {
    let mid = (ok + ng) / 2;
    if P(mid) が真 {
        ok = mid;
    } else {
        ng = mid;
    }
}
println!("{}", ok);
```