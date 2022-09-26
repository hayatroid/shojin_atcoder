use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: [Chars; 10],
    }
    let mut a = !0;
    let mut b = 0;
    let mut c = !0;
    let mut d = 0;
    for i in 0..10 {
        for j in 0..10 {
            if s[i][j] == '#' {
                a = a.min(i);
                b = b.max(i);
                c = c.min(j);
                d = d.max(j);
            }
        }
    }
    println!("{} {}", a + 1, b + 1);
    println!("{} {}", c + 1, d + 1);
}
