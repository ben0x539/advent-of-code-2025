use std::fs;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

use anyhow::Result;
use anyhow::{bail, anyhow};

fn main() -> Result<()> {
    aoc::run("day-07",
        |p| go_1(p),
        |p| go_2(p),
    )
}

fn go_1(p: &Path) -> Result<i64> {
    let f = BufReader::new(fs::File::open(&p)?);
    let mut lines = f.lines();
    let mut splits = 0;

    let mut last_line =
      lines.next().ok_or_else(|| anyhow!("no input line"))??.into_bytes();
    for line in &mut lines {
        let mut line = line?.into_bytes();
        
        for (i, &c) in last_line.iter().enumerate() {
            if c != b'S' && c != b'|' { continue; }
            match line[i] {
                b'^' => {
                    if i > 0 { line[i-1] = b'|'; }
                    if i < line.len()-1 { line[i+1] = b'|'; }
                    splits += 1;
                }
                b'.' => line[i] = b'|',
                _ => (),
            } 
        }

        last_line = line;
    }

    Ok(splits)
}

fn go_2(p: &Path) -> Result<i64> {
    let f = BufReader::new(fs::File::open(&p)?);

    let mut rays: Vec<i64> = Vec::new();
    for (l, line) in f.lines().enumerate() {
        let line = line?.into_bytes();

        if rays.is_empty() {
            rays = vec![0; line.len()];
        }
        let mut next_rays = vec![0; line.len()];
        for (i, c) in line.iter().enumerate() {
            match c {
                b'S' => next_rays[i] += 1,
                b'^' => {
                    if i > 0 {
                        next_rays[i-1] += rays[i];
                    }
                    if i < line.len()-1 {
                        next_rays[i+1] += rays[i];
                    }
                }
                _ => next_rays[i] += rays[i],
            }
        }
        rays = next_rays;
        dbg!(l, rays.iter().sum::<i64>());
    }

    Ok(rays.iter().sum::<i64>() as i64)
}