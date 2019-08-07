// imports

// [[file:~/Workspace/Programming/cmdline-tools/sbfiles/sbfiles.note::*imports][imports:1]]
use std::fs::File;
use std::io::Read;
use std::path::Path;

use quicli::prelude::*;

type Result<T> = ::std::result::Result<T, Error>;
// imports:1 ends here

// rust

// [[file:~/Workspace/Programming/cmdline-tools/sbfiles/sbfiles.note::*rust][rust:1]]
/// Add files into zip archive and encode binary data as base64 stream.
pub fn encode<P: AsRef<Path>>(files: &[P]) -> Result<String> {
    use flate2::write::GzEncoder;
    use flate2::Compression;

    // create tar.gz stream
    let buf: Vec<u8> = vec![];
    let enc = GzEncoder::new(buf, Compression::default());
    let mut tar = tar::Builder::new(enc);

    // add files into tar ball (tar.gz)
    for f in files {
        let p = f.as_ref();

        // the path in the archive is required to be relative.
        let name = if p.is_absolute() {
            p.strip_prefix("/")?
        } else {
            p
        };

        // add local files or files in directory recursively.
        if p.is_file() {
            let mut f = File::open(p)?;
            info!("archive file: {}", name.display());

            tar.append_file(name, &mut f)?;
        } else if p.is_dir() {
            tar.append_dir_all(name, p)?;
        } else {
            bail!("file does not exists: {:?}", p);
        }
    }

    // encode with base64 to plain text stream
    let data = tar.into_inner()?.finish()?;

    let b64 = base64::encode(&data);

    Ok(b64)
}
// rust:1 ends here

// rust

// [[file:~/Workspace/Programming/cmdline-tools/sbfiles/sbfiles.note::*rust][rust:1]]
/// Decode base64 encoded zip archive stream and extract all files inside.
///
/// # Parameters
///
/// * data: base64 encoded zip archive
///
pub fn decode(txt: Option<&str>) -> Result<()> {
    decode_files_to(txt, ".")
}

/// Decode base64 encoded zip archive stream and extract all files inside.
///
/// # Parameters
///
/// * data: base64 encoded zip archive
///
pub fn decode_files_to<P: AsRef<Path>>(txt: Option<&str>, path: P) -> Result<()> {
    use flate2::read::GzDecoder;
    use tar::Archive;

    // 1. decode base64 text into tar.gz stream

    // decode `txt` or the text read in from stdin.
    let tar_gz = if let Some(txt) = txt {
        base64::decode(txt)?
    } else {
        let mut buffer = String::new();
        let stdin = std::io::stdin();
        let mut handle = stdin.lock();

        handle.read_to_string(&mut buffer)?;
        base64::decode(&buffer.trim_end()).with_context(|_| format!("base64 decoding failed."))?
    };

    let tar = GzDecoder::new(tar_gz.as_slice());
    let mut archive = Archive::new(tar);
    archive.unpack(path.as_ref())?;

    Ok(())
}
// rust:1 ends here

// test

// [[file:~/Workspace/Programming/cmdline-tools/sbfiles/sbfiles.note::*test][test:1]]
#[test]
fn test_tar() -> Result<()> {
    use std::ffi::OsString;
    use std::io::Write;

    let files = vec!["Cargo.lock", "foobar.d"];
    let files: Vec<OsString> = files.iter().map(|n| n.into()).collect();

    // Create a directory inside of `std::env::temp_dir()`
    let dir = tempfile::tempdir()?;
    // make sure tempdir exists
    {
        let _ = std::env::set_current_dir(&dir)?;

        // create source files
        for f in files.iter() {
            let mut fp = File::create(&f)?;
            fp.write_all(b"test")?;
        }

        let s = encode(&files)?;
        let _ = decode_files_to(Some(&s), "x")?;

        for entry in std::fs::read_dir("x")? {
            let fname = entry?.file_name();
            assert!(files.contains(&fname));
        }
    }

    Ok(())
}
// test:1 ends here
