use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(i32, i32); n],
    }
    let mut g = vec![vec!['.'; 2001]; 2001];
    for (x, y) in xy {
        let (x, y) = ((x + 1000) as usize, (y + 1000) as usize);
        g[x][y] = '#';
    }
    let mut ans = 0;
    for i in 0..=2000 {
        for j in 0..=2000 {
            if g[i][j] == '#' {
                let mut dfs = vec![(i, j)];
                while let Some((x, y)) = dfs.pop() {
                    g[x][y] = '.';
                    for (dx, dy) in vec![(!0, !0), (!0, 0), (0, !0), (0, 1), (1, 0), (1, 1)] {
                        let (nx, ny) = (x + dx, y + dy);
                        if nx <= 2000 && ny <= 2000 && g[nx][ny] == '#' {
                            dfs.push((nx, ny));
                        }
                    }
                }
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
