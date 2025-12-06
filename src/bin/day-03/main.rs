use std::fs;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

use anyhow::Result;
use anyhow::bail;

fn main() -> Result<()> {
    aoc::run("day-03",
        |p| go(p, 2),
        |p| go(p, 12),
    )
}

fn go(p: &Path, joltage_len: usize) -> Result<i64> {
    let mut sum: i64 = 0;
    let f = BufReader::new(fs::File::open(&p)?);
    for line in f.lines() {
        let line = line?;
        if line.len() < 2 { bail!("line too short: {line}"); }
        let mut rest = &line[..];
        let mut total = 0;
        for i in 0..joltage_len {
            let (p, a) = rest[..rest.len()-(joltage_len-i-1)].bytes().enumerate()
                .max_by_key(|&(p, v)| (v, -(p as isize))).unwrap();
            rest = &rest[(p+1)..];
            total = total * 10 + (a - b'0') as i64;
        }
        sum += total;
    }

    Ok(sum)
}