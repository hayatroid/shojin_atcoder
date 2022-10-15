use proconio::input;

fn main() {
    input! {
        mut x: f64,
        k: usize,
    }
    for _ in 0..k {
        x /= 10.0;
        x = x.round();
    }
    let mut x = x as u64;
    for _ in 0..k {
        x *= 10;
    }
    println!("{}", x);
}
