use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

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

/// operate as a line-oriented stream editor, pretty-printing recognized inputs
fn process_lines() -> std::io::Result<()> {
    let opt = Opt::from_args();

    let parser = Box::new(E0277Parser::new());

    let stdin = stdin();
    let reader = stdin.lock();
    let reader = BufReader::new(reader);

    let stdout = stdout();
    let writer = stdout.lock();
    let mut writer = BufWriter::new(writer);

    for line in reader.lines() {
        let line = line?;
        match parser.parse(line.trim()) {
            Ok(item) => {
                item.pretty_to(&mut writer)?;
                write!(writer, "\n")?;
            }
            Err(err) => {
                if opt.strict {
                    writeln!(writer, "{:#?}", err)?;
                } else {
                    writeln!(writer, "{}", line)?;
                }
                if opt.fail_fast {
                    break;
                }
            }
        }
        writer.flush()?;
    }

    Ok(())
}


fn main() {
    // if there's an IO error when processing the lines, then either stdin or stdout was closed.
    // we just want a graceful exit in that case; no need to complain to the user.
    let _ = process_lines();
}
