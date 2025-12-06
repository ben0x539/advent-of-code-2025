use std::path::Path;
use std::path::PathBuf;

use anyhow::Result;

#[derive(Default, Clone, Debug, PartialEq, Eq, clap::ValueEnum)]
//#[serde(rename_all = "kebab-case")]
enum Parts {
    Part1,
    Part2,
    #[default] Both,
}

#[derive(Default, Clone, Debug, PartialEq, Eq, clap::ValueEnum)]
//#[serde(rename_all = "kebab-case")]
enum Files {
    Sample,
    Input,
    #[default] Both,
}

#[derive(Default, Clone, Debug, PartialEq, Eq)]
#[derive(clap::Parser)]
struct Opt {
    #[arg(long)]
    parts: Option<Parts>,
    #[arg(long)]
    files: Option<Files>,
    #[arg(long)]
    custom_file: Option<PathBuf>,
}

pub fn run<P, F1, F2>(p: P, part_1: F1, part_2: F2) -> Result<()>
        where
            P: AsRef<Path>,
            F1: for<'p> Fn(&'p Path) -> Result<i64>,
            F2: for<'p> Fn(&'p Path) -> Result<i64> {
    let opt = <Opt as clap::Parser>::parse();

    dbg!(&opt);
    let parts = opt.parts.unwrap_or(Parts::Both);
    let files = opt.files.unwrap_or(Files::Both);

    let mut path = PathBuf::new();
    path.push("src/bin");
    path.push(p);

    let mut results = Vec::new();

    if let Some(ref custom_file) = opt.custom_file {
        if parts != Parts::Part2 {
            eprintln!("part 1...");
            results.push(("part 1", part_1(custom_file)?));
        }

        if parts != Parts::Part1 {
            eprintln!("part 2...");
            results.push(("part 2", part_2(custom_file)?));
        }
    } else {
        let sample = path.join("sample.txt");
        let input = path.join("input.txt");

        if parts != Parts::Part2 {
            if files != Files::Input {
                eprintln!("part 1 sample...");
                results.push(("part 1 sample", part_1(&sample)?));
            }
            if files != Files::Sample {
                eprintln!("part 1 input...");
                results.push(("part 1 input", part_1(&input)?));
            }
        }

        if parts != Parts::Part1 {
            if files != Files::Input {
                eprintln!("part 2 sample...");
                results.push(("part 2 sample", part_2(&sample)?));
            }
            if files != Files::Sample {
                eprintln!("part 2 input...");
                results.push(("part 2 input", part_2(&input)?));
            }
        }
    }

    for (label, value) in results {
        println!("{label}: {value}");
    }

    Ok(())
}