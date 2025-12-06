use std::cmp;
use std::fs;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;
use std::ops::RangeInclusive;

use anyhow::Result;
use anyhow::bail;

fn main() -> Result<()> {
    aoc::run("day-05",
        |p| go(p, true),
        |p| go(p, false),
    )
}

fn go(p: &Path, check_given_ingredient_ids: bool) -> Result<i64> {
    let mut ranges: Vec<(i64, i64)> = Vec::new();
    let f = BufReader::new(fs::File::open(&p)?);
    let mut lines = f.lines();
    for line in &mut lines {
        let line = line?;
        if line.is_empty() {
            break;
        }
        let mut parts = line.split('-');
        ranges.push((parts.next().unwrap().parse()?,
            parts.next().unwrap().parse()?));
    }

    if check_given_ingredient_ids {
        let mut count = 0;
        for line in &mut lines {
            let line = line?;
            let n = line.parse()?;
            if ranges.iter().any(|&(start, end)| (start..=end).contains(&n)) {
                count += 1;
            }
        }
        Ok(count)
    } else {
        let mut count = 0;
        ranges.sort();
        let mut max = i64::MIN;
        for &(start, end) in &ranges {
            let start = cmp::max(max, start);
            if start > end { continue; }
            max = cmp::max(max, end + 1);
            count += end - start + 1;
        }
        Ok(count)
    }
}