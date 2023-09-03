use std::{
    collections::{hash_map::Entry, HashMap},
    fs::File,
    io::{BufWriter, Result, Write},
    process::Command,
};

use areacodes::{consts::*, *};

fn main() -> Result<()> {
    let clean = Command::new("git")
        .args(["diff", "--quiet"])
        .status()?
        .success();
    if !clean {
        println!("Don't do this before you stage or commit the changes!");
        return Ok(());
    }

    let file_stems = file_stems().collect::<Vec<_>>();
    for pair in file_stems.windows(2) {
        let res = Command::new("git")
            .args(["diff", "-U0", "--no-index"])
            .args([
                format!("{DATA_DIRECTORY}/{}.txt", pair[0]),
                format!("{DATA_DIRECTORY}/{}.txt", pair[1]),
            ])
            .output()?
            .stdout;

        if res.is_empty() {
            continue;
        }
        let res = unsafe { String::from_utf8_unchecked(res) };

        let mut lines = Vec::<&str>::new();
        let mut records = HashMap::<&str, usize>::new();

        for line in res.lines().skip(4) {
            if !line.starts_with(['@', '\\']) {
                match records.entry(&line[1..]) {
                    Entry::Occupied(e) => {
                        lines[*e.get()] = "";
                        continue;
                    }
                    Entry::Vacant(e) => {
                        e.insert(lines.len());
                    }
                }
                lines.push(line);
            }
        }

        let file = File::create(format!("{DIFF_DIRECTORY}/{}-{}.diff", pair[0], pair[1]))?;
        let mut bw = BufWriter::new(file);

        for line in lines {
            if !line.is_empty() {
                writeln!(bw, "{line}")?;
            }
        }
    }
    Ok(())
}

fn file_stems() -> impl Iterator<Item = String> {
    files(DATA_DIRECTORY).map(|path| path.file_stem().unwrap().to_string_lossy().into())
}
