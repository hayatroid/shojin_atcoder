fn read() -> usize {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim().parse().unwrap()
}

fn main() {
    let n = read();
    let mut l = 1;
    let mut r = n;
    while l < r {
        let mid = (l + r) / 2;
        println!("? {} {} {} {}", l, mid, 1, n);
        let res = read();
        if res < mid - l + 1 {
            r = mid
        } else {
            l = mid + 1
        }
    }
    let mut u = 1;
    let mut d = n;
    while u < d {
        let mid = (u + d) / 2;
        println!("? {} {} {} {}", 1, n, u, mid);
        let res = read();
        if res < mid - u + 1 {
            d = mid
        } else {
            u = mid + 1
        }
    }
    println!("! {} {}", l, u);
}
