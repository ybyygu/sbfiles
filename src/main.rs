// imports

// [[file:~/Workspace/Programming/cmdline-tools/sbfiles/sbfiles.note::*imports][imports:1]]
use std::path::PathBuf;
use structopt::StructOpt;

use quicli::prelude::*;
// imports:1 ends here

// cli

// [[file:~/Workspace/Programming/cmdline-tools/sbfiles/sbfiles.note::*cli][cli:1]]
/// Copy/paste files through scrollback buffer with base64 MIME encoding.
#[derive(Debug, StructOpt)]
struct Cli {
    /// Encode `files` as plain text and print it to stdout.
    #[structopt(subcommand)]
    task: Task,

    #[structopt(flatten)]
    verbosity: Verbosity,
}

#[derive(StructOpt, Debug)]
enum Task {
    /// Encode `files` as plain text and print it to stdout.
    #[structopt(name = "encode", alias = "e")]
    Encode {
        #[structopt(parse(from_os_str), required = true)]
        files: Vec<PathBuf>,
    },

    /// Decode scrollbuffer stream into files.
    #[structopt(name = "decode", alias = "d")]
    Decode {
        /// Extract files to `directory`.
        #[structopt(parse(from_os_str), long = "directory", short = "C")]
        directory: Option<PathBuf>,
    },
}

fn main() -> CliResult {
    let args = Cli::from_args();
    args.verbosity.setup_env_logger(&env!("CARGO_PKG_NAME"))?;

    match args.task {
        Task::Encode { files } => {
            let txt = sbfiles::encode(&files)?;

            print!("{}", txt);
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
