use std::fs;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

use anyhow::Result;
use anyhow::bail;

fn main() -> Result<()> {
    aoc::run("day-06",
        |p| go_1(p),
        |p| go_2(p),
    )
}

#[derive(Clone, Copy)]
enum Op { Add, Multiply }
impl Op {
    fn base(self) -> i64 {
        match self {
            Op::Add => 0,
            Op::Multiply => 1,
        }
    }

    fn apply(self, lhs: i64, rhs: i64) -> i64 {
        match self {
            Op::Add => lhs + rhs,
            Op::Multiply => lhs * rhs,
        }
    }
}

fn go_1(p: &Path) -> Result<i64> {
    let f = BufReader::new(fs::File::open(&p)?);
    let mut all_numbers = Vec::new();
    let mut ops = Vec::new();
    let mut lines = f.lines();
    for line in &mut lines {
        let line = line?;
        let iter = line.split_ascii_whitespace();
        let mut numbers: Vec<i64> = Vec::new();
        for word in iter {
            match word {
                "+" => ops.push(Op::Add),
                "*" => ops.push(Op::Multiply),
                _ => numbers.push(word.parse()?),
            }
        }
        if numbers.len() > 0 {
            all_numbers.push(numbers);
        }
    }

    let mut total = 0;
    for (i, &op) in ops.iter().enumerate() {
        match op {
            Op::Add => {
                let mut acc = 0;
                for numbers in &all_numbers {
                    acc += numbers[i];
                }
                total += acc;
            }
            Op::Multiply => {
                let mut acc = 1;
                for numbers in &all_numbers {
                    acc *= numbers[i];
                }
                total += acc;
            }
        }
    }

    Ok(total)
}

fn go_2(p: &Path) -> Result<i64> {
    let f = BufReader::new(fs::File::open(&p)?);
    let mut ops = String::new();
    let mut grid: Vec<Vec<u8>> = Vec::new();
    let mut lines = f.lines();
    for line in &mut lines {
        let line = line?;
        if line.is_empty() { continue; }
        if !line.starts_with("*") && !line.starts_with("+") {
            grid.push(line.into());
            continue;
        }
        ops = line;
    }

    let mut total = 0;
    let mut start_col = 0;
    let mut op = Op::Add;
    for (i, c) in ops.bytes().enumerate() {
        let next_op = match c {
            b'+' => Op::Add,
            b'*' => Op::Multiply,
            b' ' => continue,
            _   => bail!("unexpected op char {c:?}"),
        };

        if i > 0 {
            total += calc(op, &grid, start_col, i-1);
        }

        op = next_op;
        start_col = i;
    }
    total += calc(op, &grid, start_col, ops.len());

    Ok(total)
}

fn calc(op: Op, grid: &Vec<Vec<u8>>, start_col: usize, end_col: usize) -> i64 {
    let mut acc = op.base();
    for col in start_col..end_col {
        let mut n = 0;
        for row in grid {
            let b = row[col];
            if b == b' ' { continue; }
            n = 10 * n + (row[col] - b'0') as i64;
        }

        dbg!(acc, n);
        acc = op.apply(acc, n);
    }
    
    dbg!(acc);
    return acc;
}