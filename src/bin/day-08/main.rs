use std::collections::BTreeMap;
use std::fs;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

use anyhow::Result;
use anyhow::bail;

type Xyz = (i64, i64, i64);

struct JunctionBox {
    pos: Xyz,
    circuit: usize,
}

fn main() -> Result<()> {
    aoc::run("day-08",
        |p| go(p, false),
        |p| go(p, true),
    )
}

fn get_sqr_dist(a: Xyz, b: Xyz) -> i64 {
    let (dx, dy, dz) = (a.0 - b.0, a.1 - b.1, a.2 - b.2);
    dx*dx + dy*dy + dz*dz
}

fn go(p: &Path, connect_all: bool) -> Result<i64> {
    let f = BufReader::new(fs::File::open(&p)?);
    let mut boxes = Vec::new();

    for (i, line) in f.lines().enumerate() {
        let line = line?;
        
        let mut numbers = line.split(|c| c == ',');
        let (Some(x), Some(y), Some(z)) = (numbers.next(), numbers.next(), numbers.next()) else {
            bail!("weird line: {line}");
        };
        let xyz = (x.parse()?, y.parse()?, z.parse()?);
        boxes.push(JunctionBox {
            pos: xyz,
            circuit: i,
        });
    }

    let mut dists = Vec::new();

    for i in 0..boxes.len() {
        for j in i+1..boxes.len() {
            dists.push((get_sqr_dist(boxes[i].pos, boxes[j].pos), i, j));
        }
    }

    dists.sort();

    // 10 for sample, 1000 for real input
    let connections_limit = if boxes.len() == 20 { 10 } else { 1000 };

    let mut merges = 0;

    for (connections_count, &(_dist, i, j)) in dists.iter().enumerate() {
        if !connect_all && connections_count >= connections_limit {
            break;
        }

        let circuit_i = boxes[i].circuit;
        let circuit_j = boxes[j].circuit;

        if circuit_i == circuit_j { continue; }

        for b in &mut boxes {
            if b.circuit == circuit_j {
                b.circuit = circuit_i;
            }
        }

        if connect_all {
            merges += 1;
            if boxes.len() - merges <= 1 {
                eprintln!("last boxes are {:?}; {:?}", boxes[i].pos, boxes[j].pos);
                return Ok(boxes[i].pos.0 * boxes[j].pos.0);
            }
        }
    }

    let mut circuits = BTreeMap::new();
    for b in &boxes {
        *circuits.entry(b.circuit).or_default() += 1;
    }
    let mut circuit_sizes: Vec<_> = circuits.iter().map(|(_, &v)| v).collect();
    circuit_sizes.sort();

    Ok(circuit_sizes.iter().rev().take(3).product())
}