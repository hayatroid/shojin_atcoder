use permutohedron::factorial;
use proconio::input;

fn main() {
    input!(n: usize);
    println!("{}", factorial(n));
}
