// [[file:../sbfiles.note::5bb04e05][5bb04e05]]
use gut::cli::*;
use gut::prelude::*;

use std::path::PathBuf;
// 5bb04e05 ends here

// [[file:../sbfiles.note::da7a3420][da7a3420]]
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
        #[structopt(required = true)]
        files: Vec<PathBuf>,

        /// Write to clipboard using OSC 52 escape sequence
        #[structopt(long = "clip", short)]
        clipboard: bool,
    },

    /// Decode scrollbuffer stream into files.
    #[structopt(name = "decode", alias = "d")]
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
    let args = Cli::parse();
    args.verbosity.setup_logger();

    match args.task {
        Task::Encode { files, clipboard } => {
            let txt = sbfiles::encode(&files)?;
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
// da7a3420 ends here
