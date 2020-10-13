use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

use anyhow::Result;
use structopt::StructOpt;

use trait_bound_pretty::{
    parser::E0277Parser,
    Pretty,
};

/// Pretty-print errors E0277 for improved legibility
///
/// Reads lines from stdin. If they can be parsed as an E0277, or as an Item, then they are
/// pretty-printed to stdout. Otherwise, they are passed through unchanged.
#[derive(Debug, StructOpt)]
struct Opt {
    /// Activate strict mode
    ///
    /// Normally, any line of input which can't be parsed is passed through unchanged.
    /// In strict mode, any line of input which can't be parsed as an E0277 or an Item
    /// produces an error.
    #[structopt(short, long)]
    strict: bool,

    /// If any error is produced, abort instead of continuing at the next line.
    #[structopt(short, long)]
    fail_fast: bool,

    // /// Attempt to parse and print bare items instead of E0277 lines
    // #[structopt(short, long)]
    // bare_item: bool,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();

    let parser = Box::new(E0277Parser::new());

    let stdin = stdin();
    let reader = stdin.lock();
    let reader = BufReader::new(reader);

    let stdout = stdout();
    let writer = stdout.lock();
    let mut writer = BufWriter::new(writer);
    for line in reader.lines() {
        let line = match line {
            Ok(line) => line,
            Err(_) => {
                // io errors on reading mean that our stdin got closed;
                // just abort
                break;
            }
        };
        match parser.parse(line.trim()) {
            Ok(item) => {
                if let Err(_) = item.pretty_to(&mut writer) {
                    // io errors on writing mean that our stdout got closed;
                    // just abort
                    break;
                }
                if let Err(_) = write!(writer, "\n") {
                    break;
                }
            }
            Err(err) => {
                if opt.strict {
                    if let Err(_) = writeln!(writer, "{:#?}", err) {
                        break;
                    }
                } else {
                    if let Err(_) = writeln!(writer, "{}", line) {
                        break;
                    }
                }
                if opt.fail_fast {
                    break;
                }
            }
        }
        if let Err(_) = writer.flush() {
            break;
        }
    }

    Ok(())
}
