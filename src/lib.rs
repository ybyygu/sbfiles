// [[file:../sbfiles.note::*imports][imports:1]]
use std::fs::File;
use std::path::Path;

use gut::prelude::*;
// imports:1 ends here

// [[file:../sbfiles.note::*base][base:1]]
const MARKER_START: &str = "^^^^^^^^^^-START-OF-STREAM-^^^^^^^^";
const MARKER_END: &str = "@@@@@@@@@@@@@-END-OF-STREAM-@@@@@@@@@";
// base:1 ends here

// [[file:../sbfiles.note::5a0d1628][5a0d1628]]
/// encode binary data as text
fn base64_encode(data: &[u8]) -> String {
    let b64 = base64::encode(data);

    let mut encoded: String = MARKER_START.into();
    // encoded.push_str(&b64);
    encoded.push_str(&wrap_long_line(&b64));
    encoded.push_str(MARKER_END);
    encoded
}

/// Found encoded data block with predefined markers
fn base64_decode(txt: &str) -> Result<Vec<u8>> {
    if let Some(p0) = txt.rfind(MARKER_START) {
        let p0 = p0 + MARKER_START.len();
        if let Some(p1) = txt.rfind(MARKER_END) {
            // remove new line separator
            let b64: String = txt[p0..p1].lines().collect();
            Ok(base64::decode(&b64)?)
        } else {
            bail!("Cannot find stream end marker!");
        }
    } else {
        bail!("Cannot find stream start marker!");
    }
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
        let _ = writeln!(&mut lines, "{}", &txt[i * n..(i + 1) * n]);
    }
    let _ = writeln!(&mut lines, "{}", &txt[m * n..]);
    lines
}
// 5a0d1628 ends here

// [[file:../sbfiles.note::7b00f9a4][7b00f9a4]]
/// Add files into zip archive and encode binary data as base64 stream.
fn encode<P: AsRef<Path>>(files: &[P]) -> Result<String> {
    use flate2::write::GzEncoder;
    use flate2::Compression;

    // create tar.gz stream
    let buf: Vec<u8> = vec![];
    let enc = GzEncoder::new(buf, Compression::default());
    let mut tar = tar::Builder::new(enc);

    // for calculating relative path
    let pwd = std::env::current_dir()?;
    // add files into tar ball (tar.gz)
    for f in files {
        let p = f.as_ref();

        // the path in the archive is required to be relative.
        let name = if p.is_absolute() {
            if p.starts_with(&pwd) {
                p.strip_prefix(&pwd)?
            } else {
                p.strip_prefix("/")?
            }
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

    Ok(base64_encode(&data))
}
// 7b00f9a4 ends here

// [[file:../sbfiles.note::802bcea7][802bcea7]]
use flate2::read::GzDecoder;
use tar::Archive;

/// Decode base64 encoded zip archive stream and extract all files inside.
///
/// # Parameters
///
/// * data: base64 encoded zip archive
///
fn decode(txt: Option<&str>) -> Result<()> {
    decode_files_to(txt, ".")
}

/// Decode base64 encoded zip archive stream and extract all files inside.
///
/// # Parameters
///
/// * data: base64 encoded zip archive
///
fn decode_files_to<P: AsRef<Path>>(txt: Option<&str>, path: P) -> Result<()> {
    use std::io::BufRead;

    // decode `txt` or the text read in from stdin.
    let txt = if let Some(txt) = txt {
        txt.to_owned()
    } else {
        // handle wrapped lines
        let mut buffer = String::new();
        for line in std::io::stdin().lock().lines() {
            let line = line?;
            buffer.push_str(&line);
        }
        buffer
    };

    // decode base64 text into tar.gz stream
    let tar_gz = base64_decode(&txt)?;
    let mut archive = decode_tar_gz(tar_gz.as_slice())?;
    archive.unpack(path.as_ref())?;

    Ok(())
}

/// decode tar.gz stream as tar stream
fn decode_tar_gz(tar_gz: &[u8]) -> Result<Archive<GzDecoder<&[u8]>>> {
    let tar = GzDecoder::new(tar_gz);
    Ok(Archive::new(tar))
}
// 802bcea7 ends here

// [[file:../sbfiles.note::c8fe874c][c8fe874c]]
/// Copy/paste files using terminal scrollback buffer
pub struct Sbfiles {
    decoded_tar_gz: Vec<u8>,
}

impl Sbfiles {
    pub fn new() -> Self {
        Self {
            decoded_tar_gz: Vec::new(),
        }
    }

    /// Decode base64 text as tar ball stream
    pub fn decode_as_tar(&mut self, txt: &str) -> Result<Archive<GzDecoder<&[u8]>>> {
        self.decoded_tar_gz = base64_decode(txt)?;
        decode_tar_gz(self.decoded_tar_gz.as_slice())
    }

    /// Encode files as compressed tar.gz base64 text
    pub fn encode<P: AsRef<Path>>(files: &[P]) -> Result<String> {
        encode(files)
    }

    /// Decode base64 encoded zip archive stream and extract all files inside.
    ///
    /// # Parameters
    /// * data: base64 encoded zip archive
    pub fn decode(txt: Option<&str>) -> Result<()> {
        decode_files_to(txt, ".")
    }

    /// Decode base64 encoded zip archive stream and extract all files inside.
    ///
    /// # Parameters
    ///
    /// * data: base64 encoded zip archive
    pub fn decode_files_to<P: AsRef<Path>>(txt: Option<&str>, path: P) -> Result<()> {
        decode_files_to(txt, path)
    }
}
// c8fe874c ends here

// [[file:../sbfiles.note::*test][test:1]]
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
