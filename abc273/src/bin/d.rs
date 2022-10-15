use proconio::input;
use std::collections::{BTreeMap, BTreeSet};

fn main() {
    input! {
        h: i32,
        w: i32,
        mut pos: (i32, i32),
        rc: [(i32, i32)],
        dl: [(char, i32)],
    }
    let mut row = BTreeMap::new();
    let mut col = BTreeMap::new();
    for (r, c) in rc {
        row.entry(c).or_insert(BTreeSet::new()).insert(r);
        col.entry(r).or_insert(BTreeSet::new()).insert(c);
    }
    for (d, l) in dl {
        row.entry(pos.1).or_insert(BTreeSet::new());
        col.entry(pos.0).or_insert(BTreeSet::new());
        let left = *col[&pos.0].range(..pos.1).next_back().unwrap_or(&0);
        let right = *col[&pos.0].range(pos.1..).next().unwrap_or(&(w + 1));
        let up = *row[&pos.1].range(..pos.0).next_back().unwrap_or(&0);
        let down = *row[&pos.1].range(pos.0..).next().unwrap_or(&(h + 1));
        match d {
            'L' => pos.1 = (left + 1).max(pos.1 - l),
            'R' => pos.1 = (right - 1).min(pos.1 + l),
            'U' => pos.0 = (up + 1).max(pos.0 - l),
            'D' => pos.0 = (down - 1).min(pos.0 + l),
            _ => unreachable!(),
        }
        println!("{} {}", pos.0, pos.1);
    }
}
