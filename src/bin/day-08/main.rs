use std::collections::BTreeMap;
use std::fs;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

use anyhow::Result;
use anyhow::bail;

type Xyz = (i64, i64, i64);

#[derive(Debug)]
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

fn resolve_circuit(boxes: &mut Vec<JunctionBox>, i: usize) -> usize {
    let mut p = boxes[i].circuit;
    loop {
        if p == boxes[p].circuit {
            break;
        }

        p = boxes[p].circuit;
    }

    let mut i = i;
    while i != p {
        let q = boxes[i].circuit;
        boxes[i].circuit = p;
        i = q;
    }

    return p;
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

    //for v in boxes.iter().enumerate() {
    //    eprintln!("{v:?}");
    //}

    for (connections_count, &(_dist, i, j)) in dists.iter().enumerate() {
        if !connect_all && connections_count >= connections_limit {
            break;
        }

        let circuit_i = resolve_circuit(&mut boxes, i);
        let circuit_j = resolve_circuit(&mut boxes, j);

        if circuit_i == circuit_j {
            //eprintln!("boxes {i}, {j} are already both in circuit {circuit_i}");
            continue;
         }

        //eprintln!("connecting boxes {i}, {j} in circuit {circuit_i}");
        boxes[circuit_j].circuit = circuit_i;
        //eprintln!("{:?}", boxes.iter().map(|b| b.circuit).collect::<Vec<_>>());

        if connect_all {
            merges += 1;
            if boxes.len() - merges <= 1 {
                //eprintln!("last boxes are {:?}; {:?}", boxes[i].pos, boxes[j].pos);
                return Ok(boxes[i].pos.0 * boxes[j].pos.0);
            }
        }
    }

    let mut circuits = BTreeMap::new();
    for i in 0..boxes.len() {
        *circuits.entry(resolve_circuit(&mut boxes, i)).or_default() += 1;
    }
    let mut circuit_sizes: Vec<_> = circuits.iter().map(|(_, &v)| v).collect();
    //dbg!(&circuit_sizes);
    circuit_sizes.sort();

    //for v in boxes.iter().enumerate() {
    //    eprintln!("{v:?}");
    //}

    Ok(circuit_sizes.iter().rev().take(3).product())
}