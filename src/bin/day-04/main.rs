
use std::fs;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

use anyhow::Result;
use anyhow::bail;

fn main() -> Result<()> {
    aoc::run("day-04",
        |p| go(p, false),
        |p| go(p, true),
    )
}

fn go(p: &Path, keep_going: bool) -> Result<i64> {
    let f = BufReader::new(fs::File::open(&p)?);
    let mut grid: Vec<Vec<u8>> = Vec::new();
    for line in f.lines() {
        let line = line?;
        grid.push(line.into());
    }
    if grid.len() == 0 { bail!("empty input"); }

    let mut total_removed = 0;
    loop {
        let mut inaccessible: i64 = 0;
        let mut total_rolls = 0;

        let mut counts = vec![vec![0; grid[0].len()]; grid.len()];
        for (y, row) in grid.iter().enumerate() {
            for (x, &v) in row.iter().enumerate() {
                if v != b'@' { continue; }
                total_rolls += 1;
                for dy in -1..=1isize {
                    let py = y.wrapping_add(dy as usize);
                    if py >= grid.len() { continue; }
                    for dx in -1..=1isize {
                        if dx == 0 && dy == 0 { continue; }
                        let px = x.wrapping_add(dx as usize);
                        if px >= row.len() { continue; }

                        if grid[py][px] != b'@' { continue; }
                        counts[py][px] += 1;
                        if counts[py][px] == 4 {
                            inaccessible += 1;
                        }
                    }
                }
            }
        }

        for (y, row) in grid.iter_mut().enumerate() {
            for (x, v) in row.iter_mut().enumerate() {
                if *v == b'@' {
                    if counts[y][x] < 4 {
                        *v = b'_';
                        print!("x");
                    } else {
                        print!("@");
                    }
                } else {
                    print!(".");
                }
            }
            println!("");
        }

        let removed = total_rolls - inaccessible;
        total_removed += removed;
        if removed == 0 || !keep_going {
            break;
        }
    }
    Ok(total_removed)
}