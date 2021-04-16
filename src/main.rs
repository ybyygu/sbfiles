// [[file:../sbfiles.note::*imports][imports:1]]
use std::path::PathBuf;
use structopt::StructOpt;

use gut::prelude::*;
// imports:1 ends here

// [[file:../sbfiles.note::*cli][cli:1]]
/// Copy/paste files through scrollback buffer with base64 MIME encoding.
#[derive(Debug, StructOpt)]
struct Cli {
    /// Encode `files` as plain text and print it to stdout.
    #[structopt(subcommand)]
    task: Task,

    #[structopt(flatten)]
    verbosity: gut::cli::Verbosity,
}

#[derive(StructOpt, Debug)]
enum Task {
    /// Encode `files` as plain text and print it to stdout.
    #[structopt(name = "encode", alias = "e")]
    Encode {
        #[structopt(parse(from_os_str), required = true)]
        files: Vec<PathBuf>,

        /// Write to clipboard using OSC 52 escape sequence
        #[structopt(long = "clip", short)]
        clipboard: bool,
    },

    /// Decode scrollbuffer stream into files.
    #[structopt(name = "decode", alias = "d")]
    Decode {
        /// Extract files to `directory`.
        #[structopt(parse(from_os_str), long = "directory", short = "C")]
        directory: Option<PathBuf>,
    },
}

// Use OSC 52 escape sequence to set clipboard through stdout
//
// Reference:
// https://github.com/sunaku/home/blob/master/bin/yank
fn copy_to_clipboard(txt: &str) -> Result<()> {
    println!("Wring to clipboard using OSC 52 escape sequence.");

    print!("\x1B]52;c;{}\x07", base64::encode(txt));
    Ok(())
}

// wrap the long string into multiple lines.
//
// 76 is the default value in the standard program 'base64'
//
// another way is to use email.base64MIME.encode
fn wrap_long_line(txt: &str) -> String {
    let n = 76;
    let mut lines = String::new();

    let m = (txt.len() as f64 / n as f64) as usize;
    for i in 0..m {
        writeln!(&mut lines, "{}", &txt[i * n..(i + 1) * n]);
    }
    writeln!(&mut lines, "{}", &txt[m * n..]);
    lines
}

fn main() -> gut::cli::CliResult {
    let args = Cli::from_args();
    args.verbosity.setup_logger();

    match args.task {
        Task::Encode { files, clipboard } => {
            let txt = sbfiles::encode(&files)?;
            let txt = wrap_long_line(&txt);
            if clipboard {
                copy_to_clipboard(&txt)?;
            } else {
                println!("{}", &txt);
            }
        }
        Task::Decode { directory } => {
            println!("Paste encoded files stream here. Press Ctrl-d to execute.");
            let stream = None;
            if let Some(d) = directory {
                let _ = sbfiles::decode_files_to(stream, d)?;
            } else {
                let _ = sbfiles::decode(stream)?;
            }
        }
    }
    Ok(())
}
// cli:1 ends here
