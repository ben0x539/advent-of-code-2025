
use std::fs;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

use anyhow::Result;
use anyhow::bail;

fn main() -> Result<()> {
    aoc::run("day-01",
        |p| go(p, |pos, n| if (pos + n).rem_euclid(100) == 0 { 1 } else { 0 }),
        |p| go(p, |pos, n| {
            let n_ = n % 100;
            (n.abs() / 100)
            + if (pos > 0 && n_ <= -pos) || (pos + n_) >= 100 { 1 } else { 0 }
        })
    )
}

fn go<F: Fn(i64, i64) -> i64>(p: &Path, check: F) -> Result<i64> {
    let mut count = 0;
    let mut pos: i64 = 50;
    let f = BufReader::new(fs::File::open(&p)?);
    for line in f.lines() {
        let mut line: Vec<u8> = line?.into();
        line[0] = match line.get(0) {
            Some(b'L') => b'-',
            Some(b'R') => b'+',
            _ => bail!("bad line {}", String::try_from(line)?),
        };
        let line = String::try_from(line)?;
        let n: i64 = line.parse()?;
        // dbg!(&line, pos, n, check(pos, n));
        count += check(pos, n);
        pos = (pos + n).rem_euclid(100);
    }

    Ok(count)
}