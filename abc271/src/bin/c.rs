use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
    }
    let mut ok = 0;
    let mut ng = n + 1;
    while ng - ok > 1 {
        let mid = (ok + ng) / 2;
        let mut f = vec![false; mid];
        let mut tmp = 0;
        for &a in &a {
            if a < mid && !f[a] {
                f[a] = true;
            } else {
                tmp += 1;
            }
        }
        tmp -= 2 * f.iter().filter(|&f| !f).count() as i32;
        if tmp >= 0 {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    println!("{}", ok);
}
