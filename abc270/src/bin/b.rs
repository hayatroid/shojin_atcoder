use proconio::input;

fn main() {
    input! {
        mut x: i32,
        mut y: i32,
        mut z: i32,
    }
    if y < 0 {
        x = -x;
        y = -y;
        z = -z;
    }
    let ans = if x < y {
        x.abs()
    } else if y < z {
        -1
    } else {
        z.abs() + (x - z).abs()
    };
    println!("{}", ans);
}
