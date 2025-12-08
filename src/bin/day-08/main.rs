use std::cell::RefCell;
use std::collections::BTreeSet;
use std::fs;
use std::io::BufRead;
use std::io::BufReader;
use std::mem;
use std::path::Path;
use std::rc::Rc;

use anyhow::Result;
use anyhow::bail;

type Xyz = (i64, i64, i64);

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Circuit {
    box_ids: Vec<usize>,
}

struct JunctionBox {
    pos: Xyz,
    circuit: Rc<RefCell<Circuit>>,
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
            circuit: Rc::new(RefCell::new(Circuit { 
                box_ids: vec![i],
            })),
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

    for (connections_count, &(dist, i, j)) in dists.iter().enumerate() {
        if !connect_all && connections_count >= connections_limit {
            break;
        }

        let mut circuit_i = Rc::clone(&boxes[i].circuit);
        let mut circuit_j = Rc::clone(&boxes[j].circuit);

        if Rc::ptr_eq(&circuit_i, &circuit_j) { continue; }

            //eprintln!("circuit {} subsumes {}", circuit_i[0], circuit_j[0]);
        if circuit_i.borrow().box_ids.len() < circuit_j.borrow().box_ids.len() {
            mem::swap(&mut circuit_i, &mut circuit_j);
        }

        circuit_i.borrow_mut().box_ids.extend_from_slice(&circuit_j.borrow().box_ids);
        for &k in &circuit_j.borrow().box_ids {
            boxes[k].circuit = Rc::clone(&circuit_i);
        }

        if connect_all && circuit_i.borrow().box_ids.len() == boxes.len() {
            eprintln!("last boxes are {:?}; {:?}", boxes[i].pos, boxes[j].pos);
            return Ok(boxes[i].pos.0 * boxes[j].pos.0);
        }
    }

    let mut circuits = BTreeSet::new();
    for b in &boxes {
        circuits.insert((b.circuit.borrow().box_ids.len(), Rc::clone(&b.circuit)));
    }

    Ok(circuits.iter().rev().take(3).map(|&(len, _)| len as i64).product())
}