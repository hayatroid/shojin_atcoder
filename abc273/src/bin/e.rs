use im_rc::Vector;
use itertools::Itertools;
use proconio::{fastout, input};
use std::collections::BTreeMap;

#[fastout]
fn main() {
    input!(q: usize);
    let mut ans = vec![];
    let mut map = BTreeMap::new();
    let mut v = Vector::new();
    for _ in 0..q {
        input!(op: String);
        if op == "ADD".to_string() {
            input!(x: i32);
            v.push_back(x);
        }
        if op == "DELETE".to_string() {
            v.pop_back();
        }
        if op == "SAVE".to_string() {
            input!(y: i32);
            map.insert(y, v.clone());
        }
        if op == "LOAD".to_string() {
            input!(z: i32);
            v = map.get(&z).cloned().unwrap_or(Vector::new());
        }
        ans.push(v.last().copied().unwrap_or(-1));
    }
    println!("{}", ans.iter().join(" "));
}
