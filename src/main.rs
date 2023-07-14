// [[file:../sbfiles.note::5bb04e05][5bb04e05]]
use gut::cli::*;
use gut::prelude::*;

use std::path::PathBuf;
// 5bb04e05 ends here

// [[file:../sbfiles.note::da7a3420][da7a3420]]
/// Copy/paste files through scrollback buffer with base64 MIME encoding.
#[derive(Debug, Parser)]
struct Cli {
    /// Encode `files` as plain text and print it to stdout.
    #[structopt(subcommand)]
    task: Task,

    #[structopt(flatten)]
    verbosity: gut::cli::Verbosity,
}

#[derive(Parser, Debug)]
enum Task {
    /// Encode `files` as plain text and print it to stdout.
    #[clap(name = "encode", alias = "e")]
    Encode {
        files: Vec<PathBuf>,

        /// Write to clipboard using OSC 52 escape sequence
        #[structopt(long = "clip", short)]
        clipboard: bool,
    },

    /// Decode scrollbuffer stream into files.
    #[clap(name = "decode", alias = "d")]
    Decode {
        /// Extract files to `directory`.
        #[structopt(long = "directory", short = 'C')]
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

fn main() -> Result<()> {
    use sbfiles::Sbfiles;
    use std::io::BufRead;

    let args = Cli::parse();
    args.verbosity.setup_logger();

    match args.task {
        Task::Encode { mut files, clipboard } => {
            if files.is_empty() {
                // Get a handle to stdin
                let stdin = std::io::stdin();
                // Lock stdin for reading
                let mut stdin = stdin.lock();
                for line in stdin.lines() {
                    let line = line?;
                    files.push(PathBuf::from(&line));
                }
            };
            let txt = Sbfiles::encode(&files)?;
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
                let _ = Sbfiles::decode_files_to(stream, d)?;
            } else {
                let _ = Sbfiles::decode(stream)?;
            }
        }
    }
    Ok(())
}
// da7a3420 ends here
