use std::io::{stdin, stdout, BufRead, BufReader};

use color_eyre::eyre::Result;
use trait_bound_pretty::parser::ItemParser;

fn main() -> Result<()> {
    color_eyre::install()?;

    let parser = ItemParser::new();

    let stdin = stdin();
    let reader = stdin.lock();
    let reader = BufReader::new(reader);

    let stdout = stdout();
    let mut writer = stdout.lock();
    for line in reader.lines() {
        let line = line?;
        let line = line.trim();
        let item = parser.parse(line)?;
        item.pretty_to(&mut writer)?;
    }
    Ok(())
}
