use std::fs;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

use anyhow::Result;
use anyhow::bail;

fn main() -> Result<()> {
    aoc::run("day-09",
        |p| go(p, false),
        |p| go(p, true),
    )
}

fn pairs<I>(i: I) -> impl Iterator<Item=(I::Item, I::Item)>
        where I: Iterator+Clone {
    i.clone().zip(i.clone().skip(1).chain(i.take(1)))
}

fn go(p: &Path, bounded: bool) -> Result<i64> {
    let f = BufReader::new(fs::File::open(&p)?);

    let mut red_tiles: Vec<(i64, i64)> = Vec::new();

    for line in f.lines() {
        let line = line?;

        let mut numbers = line.split(|c| c == ',');
        let (Some(x), Some(y)) = (numbers.next(), numbers.next()) else {
            bail!("weird line: {line}");
        };

        red_tiles.push((x.parse()?, y.parse()?));
    }

    let mut adjusts = vec![(0, 0); red_tiles.len()];
    //let pair_iter = red_tiles.iter()
    //    .zip(red_tiles.iter().skip(1)
    //        .chain(red_tiles.iter().take(1)))
    //    .enumerate();
    for (i, (&(x1, y1), &(x2, y2))) in pairs(red_tiles.iter()).enumerate() {
        let j = (i+1)%adjusts.len();
        //let (dx, dy) = (i64::signum(x2-x1), i64::signum(y2-y1));
        // adjusts[i].0 -= dy;
        // adjusts[i].1 -= dx;
        // adjusts[j].0 -= dy;
        // adjusts[j].1 -= dx;
        match (i64::signum(x2-x1), i64::signum(y2-y1)) {
            (1, 0) => { adjusts[i].1 -= 0; adjusts[j].1 -= 0; }
            (0, 1) => { adjusts[i].0 -= 0; adjusts[j].0 -= 0; }
            (-1, 0) => { adjusts[i].1 += 1; adjusts[j].1 += 1; }
            (0, -1) => { adjusts[i].0 += 1; adjusts[j].0 += 1; }
            _ => { panic!("i checked and this doesn't happen"); }
        }
    }

    //for ((x, y), &(ax, ay)) in red_tiles.iter_mut().zip(adjusts.iter()) {
    //    *x += ax;
    //    *y += ay;
    //}

    //{
    //    use std::io::Write;
    //    let mut s = File::create(p.with_extension("svg"))?;
    //    write!(s, r#"<svg width="100" height="100" xmlns="http://www.w3.org/2000/svg"><path d=""#)?;
    //    let &(x, y) = red_tiles.first().unwrap();
    //    write!(s, "M {x} {y}")?;
    //    for (x, y) in red_tiles[1..].iter() {
    //        write!(s, " L {x} {y}")?;
    //    }
    //    write!(s, r#" Z"/></svg>"#)?;
    //}

    let mut areas = Vec::new();
    for (i, &(x1, y1)) in red_tiles.iter().enumerate() {
        'j: for j in i+1..red_tiles.len() {
            let (x2, y2) = red_tiles[j];

            if (y1 < 49000) != (y2 < 49000) { continue; } // lmfao

            if bounded {
                let x_min = i64::min(x1, x2);
                let x_max = i64::max(x1, x2)+1;
                let y_min = i64::min(y1, y2);
                let y_max = i64::max(y1, y2)+1;
                for k in 0..red_tiles.len() {
                //for k in (i+1..j).chain(j+1..red_tiles.len()).chain(0..i) {
                    if k == i || k == j { continue; }
                    let (x3, y3) = red_tiles[k];
                    let (ax3, ay3) = adjusts[k];
                    if (x_min+1..x_max).contains(&(x3+ax3))
                            && (y_min+1..y_max).contains(&(y3+ay3)) {
                        continue 'j;
                    }
                }
            }

            let (dx, dy) = (i64::abs(x1 - x2)+1, i64::abs(y1 - y2)+1);
            areas.push((dx*dy, i, j));
        }
    }

    areas.sort();

    let &(area, i, j) = areas.last().unwrap();
    dbg!(area, i, j, red_tiles[i], red_tiles[j]);

    Ok(areas.last().map(|&(d, _, _)| d).unwrap_or(0))
}