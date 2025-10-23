use ::std::env;
use ::std::io::{self, Read, Write};
use clap::{Arg, Command};
use std::io::ErrorKind;

const CHUNK_SIZE: usize = 16 * 1024;

fn main() -> Result<(), ErrorKind> {
    let mut total_bytes = 0;
    let mut buffer = [0; CHUNK_SIZE];
    loop {
        let matches = Command::new("pipeviewer")
            .arg(
                Arg::new("infile")
                    .value_name("infile")
                    .help("Read input from a file instead of stdin"),
            )
            .arg(
                Arg::new("outfile")
                    .value_name("outfile")
                    .short('s')
                    .long("outfile")
                    .help("Write output to a file instead of stdout"),
            )
            .arg(
                Arg::new("silent")
                    .short('s')
                    .long("silent")
                    .help("Don't print progress"),
            )
            .get_matches();
        let infile = matches.get_one("infile").unwrap_or_default();
        let outfile = matches.get_one("outfile").unwrap_or_default();

        let mut silent = matches.get_flag("silent");

        if silent {
            silent = !env::var("PV_SILENT").unwrap_or_default().is_empty()
        };

        dbg!(silent, infile, outfile);

        let silent = env::var("PV_SILENT").unwrap_or_default().is_empty();
        dbg!(silent);
        let num_read = match io::stdin().read(&mut buffer) {
            Ok(0) => break,
            Ok(n) => n,
            Err(err) => {
                eprintln!("Error reading from stdin: {}", err);
                break;
            }
        };

        dbg!(total_bytes += num_read);
        eprintln!("Number of Bytes read: {}", total_bytes);
        if let Err(e) = io::stdout().write_all(&buffer[..num_read]) {
            if e.kind() == ErrorKind::BrokenPipe {
                break;
            }
            return Err(e.kind());
        }
        if !silent {
            eprint!("\r{}", total_bytes);
            io::stdout().flush().unwrap();
        }
    }
    Ok(())
}
