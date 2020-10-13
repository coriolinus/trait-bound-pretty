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
        let item = parser
            .parse(line)
            // the token is bounded on the input lifetime in the error case, and
            // doesn't live long enough to be returned from main, so let's take
            // ownership of it so this is possible.
            .map_err(|err| err.map_token(|token| token.to_string()))?;
        item.pretty_to(&mut writer)?;
    }
    Ok(())
}
