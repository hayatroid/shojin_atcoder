use proconio::input;

fn main() {
    input! {
        n: usize,
        l: u32,
        k: usize,
        mut a: [u32; n],
    }
    a.push(l);
    let mut ok = 0;
    let mut ng = l;
    while ng - ok > 1 {
        let mid = (ok + ng) / 2;
        let mut cnt = 0;
        let mut pre = 0;
        for &a in &a {
            if a - pre >= mid {
                cnt += 1;
                pre = a;
            }
        }
        if cnt >= k + 1 {
            ok = mid
        } else {
            ng = mid
        }
    }
    println!("{}", ok);
}
