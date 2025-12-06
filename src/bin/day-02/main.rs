use std::fs;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

use anyhow::Result;
use anyhow::bail;

fn main() -> Result<()> {
    aoc::run("day-02",
        |p| go(p, is_invalid_id_part_1),
        |p| go(p, is_invalid_id_part_2),
    )
}

fn is_invalid_id_part_1(id: i64) -> bool {
    let s = id.to_string();
    if s.len() % 2 != 0 { return false; }
    let (a, b) = s.split_at(s.len() / 2);
    return a == b;
}

fn is_invalid_id_part_2(id: i64) -> bool {
    let s = id.to_string();
    for l in 1..=(s.len() / 2) {
        if s.len() % l != 0 { continue; }
        let (a, b) = s.split_at(l);
        if b.trim_start_matches(a).is_empty() {
            return true;
        }
    }

    return false;
}

fn go(p: &Path, check: fn(i64) -> bool) -> Result<i64> {
    let mut sum: i64 = 0;
    let mut f = BufReader::new(fs::File::open(&p)?);
    let mut buf = Vec::new();
    loop {
        buf.clear();
        f.read_until(b',', &mut buf)?;
        if buf.is_empty() { break; }
        if buf.last() == Some(&b',') { buf.pop(); }
        let range = String::from_utf8(buf)?;
        let mut i = range.split(|c| c == '-');
        let (Some(start), Some(end), None) = (i.next(), i.next(), i.next()) else {
            bail!("weird range: {}", range);
        };
        dbg!(start, end);
        for id in start.parse()?..=end.parse()? {
            if check(id) {
                sum += id;
            }
        }
        buf = range.into();
    }

    Ok(sum)
}