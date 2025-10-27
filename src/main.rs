use ::std::env;
use clap::{Arg, Command};
use std::fs::File;
use std::io::{Error, ErrorKind};
use std::io::{self, Read, Write};

const CHUNK_SIZE: usize = 16 * 1024;

fn main() -> io::Result<()> {
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
        // let infile = matches.get_one("infile").unwrap_or_default();
        // let outfile = matches.get_one("outfile").unwrap_or_default();
        // let mut silent = matches.get_flag("silent");
        //
        // if silent {
        //     silent = !env::var("PV_SILENT").unwrap_or_default().is_empty()
        // };
        //
        // let mut reader:Box<dyn ,&dyn Read> = if !infile.is_empty() {
        //     Box::new(File::open(infile)?)
        // } else {
        //     Box::new(io::stdin())
        // };
        //
        // let mut writer: Box<dyn , Write> = if !outfile.is_empty() {
        //     Box::new(File::create(outfile)?)
        // } else {
        //     Box::new(io::stdout())
        // };

        let infile = matches.get_one::<String>("infile").map_or(String::new(), |s| s.to_string());
        let outfile = matches.get_one::<String>("outfile").map_or(String::new(), |s| s.to_string());

        let mut silent = matches.get_flag("silent");
        if !silent {
            silent = !env::var("PV_SILENT").unwrap_or_default().is_empty();
        }

        let mut reader: Box<dyn Read> = if !infile.is_empty() {
            Box::new(File::open(infile)?)
        } else {
            Box::new(io::stdin())
        };

        let mut writer: Box<dyn Write> = if !outfile.is_empty() {
            Box::new(File::create(outfile)?)
        } else {
            Box::new(io::stdout())
        };


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

        total_bytes += num_read;
        eprintln!("Number of Bytes read: {}", total_bytes);
        if let Err(e) = writer.write_all(&buffer[..num_read]) {
            if e.kind() == ErrorKind::BrokenPipe {
                break;
            }
            return Err(Error::from(e.kind()));
        }
        if !silent {
            eprint!("\r{}", total_bytes);
            io::stdout().flush()?;
        }
    }
    Ok(())
}
