// imports

// [[file:~/Workspace/Programming/cmdline-tools/sbfiles/sbfiles.note::*imports][imports:1]]
use std::ffi::OsString;
use std::path::{Path, PathBuf};

use quicli::prelude::*;

type Result<T> = ::std::result::Result<T, Error>;

use duct::cmd;
// imports:1 ends here

// encode

// [[file:~/Workspace/Programming/cmdline-tools/sbfiles/sbfiles.note::*encode][encode:1]]
/// Add files into zip archive and encode binary data as base64 stream.
pub fn encode<P: AsRef<Path>>(files: &[P]) -> Result<String> {
    assert!(!files.is_empty(), "empty list of files!");

    // tar --create --gzip --verbose --file -
    let mut args: Vec<PathBuf> = vec!["--create", "--gzip", "--verbose", "--file", "-"]
        .into_iter()
        .map(|s| s.into())
        .collect();

    // add files to tar ball.
    for p in files {
        args.push(p.as_ref().into());
    }
    let x = cmd("tar", &args).pipe(cmd!("base64")).read()?;

    Ok(x)
}
// encode:1 ends here

// decode

// [[file:~/Workspace/Programming/cmdline-tools/sbfiles/sbfiles.note::*decode][decode:1]]
/// Decode base64 encoded zip archive stream and extract all files inside.
///
/// # Parameters
///
/// * data: base64 encoded zip archive
///
pub fn decode(txt: Option<&str>) -> Result<()> {
    // base64 -d | tar --extract --verbose --gzip --file -
    let d = if let Some(txt) = txt {
        cmd!("base64", "--decode").input(txt)
    } else {
        cmd!("base64", "--decode")
    };
    let x = d
        .pipe(cmd!(
            "tar",
            "--extract",
            "--verbose",
            "--gzip",
            "--file",
            "-"
        ))
        .read();
    dbg!(x);

    Ok(())
}
// decode:1 ends here

// test

// [[file:~/Workspace/Programming/cmdline-tools/sbfiles/sbfiles.note::*test][test:1]]
#[test]
fn test_tar() -> Result<()> {
    let files = vec!["/tmp/login.log", "/home/ybyygu/gss.svg"];

    // Create a directory inside of `std::env::temp_dir()`
    let dir = tempfile::tempdir()?;
    // make sure tempdir exists
    {
        let _ = std::env::set_current_dir(&dir)?;

        let s = encode(&files)?;
        let _ = decode(Some(&s))?;

        let x = cmd!("fd").read()?;
        dbg!(x);
    }

    Ok(())
}
// test:1 ends here
