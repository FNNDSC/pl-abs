use std::{fs, io};
use std::io::{Read, Write, BufReader, BufWriter};
use std::path::Path;

const NUMBER_STARTS: [u8; 11] = [
    b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'0', b'.'
];

/// A low-level implementation of absolute value, removing negative signs which precede numbers.
/// We avoid deserializing the data as floats to avoid loss of precision.
pub fn abs_file(input_file: &Path, output_file: &Path) -> io::Result<()> {
    let reader = fs::File::open(input_file).map(BufReader::new)?;
    let mut writer = fs::File::create(output_file).map(BufWriter::new)?;

    let mut prev = None;
    let mut was_negative = false;
    let mut cur = b'\0';

    for nb in reader.bytes() {
        cur = nb?;
        if cur == b'-' {
            was_negative = true;
        } else if was_negative {
            if NUMBER_STARTS.iter().any(|n| n == &cur) {
                prev = None;
            }
            was_negative = false;
        }
        if let Some(p) = prev {
            writer.write(&[p])?;
        }
        prev = Some(cur)
    }

    if cur != b'\0' {
        writer.write(&[cur])?;
    }
    writer.flush()?;
    Ok(())
}
