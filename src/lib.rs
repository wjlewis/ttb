use std::io::{BufWriter, Read, Write};

/// Read bytes from `reader`, and write the appropriate bytes to `writer`,
/// transforming `b'0'` to `0` and `b'1'` to `1`. All other bytes are ignored.
/// Additionally, if `b';'` is encountered, all bytes until the next `CR` or
/// `LF` are ignored. This allows for comments which might contain `b'0'` or
/// `b'1'`.
pub fn process<R: Read, W: Write>(reader: R, writer: W) {
    let mut writer = BufWriter::new(writer);

    // TODO Handle read errors.
    let mut reader = reader.bytes().filter_map(Result::ok).peekable();

    let mut current_byte = 0;
    let mut byte_index = 7;

    while let Some(b) = reader.peek() {
        match b {
            b'0' | b'1' => {
                let b = if reader.next().unwrap() == b'0' { 0 } else { 1 };
                let b = b << byte_index;
                current_byte |= b;

                if byte_index == 0 {
                    // Done with this byte; write it.
                    writer
                        .write_all(&[current_byte])
                        .expect("error writing byte");
                    current_byte = 0;
                    byte_index = 7;
                } else {
                    byte_index -= 1;
                }
            }
            b';' => {
                reader.next();
                while let Some(b) = reader.peek() {
                    match b {
                        b'\n' | b'\r' => break,
                        _ => {
                            reader.next();
                        }
                    }
                }
            }
            _ => {
                reader.next();
            }
        }
    }

    writer.flush().expect("error flushing writer");
}

#[cfg(test)]
mod tests {
    use super::process;

    #[test]
    fn smoke_1() {
        let mut input = String::from("0000_0000__1111_1111");
        let reader = unsafe { input.as_mut_vec() };
        let mut writer: Vec<u8> = Vec::new();

        process(&mut reader.as_slice(), &mut writer);

        assert_eq!(writer, vec![0x00, 0xff]);
    }

    #[test]
    fn smoke_2() {
        let mut input = String::from(
            r#";; A binary file
00000000_00000001 ;; 0 1
00000010_00000011 ;; 2 3
00000100_00000101 ;; 4 5
00000110_00000111 ;; 6 7
00001000_00001001 ;; 8 9
00001010_00001011 ;; a b
00001100_00001101 ;; c d
00001110_00001111 ;; e f
"#,
        );
        let reader = unsafe { input.as_mut_vec() };
        let mut writer: Vec<u8> = Vec::new();

        process(&mut reader.as_slice(), &mut writer);

        assert_eq!(
            writer,
            vec![0x0, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9, 0xa, 0xb, 0xc, 0xd, 0xe, 0xf]
        );
    }

    #[test]
    fn comments() {
        let mut input = String::from("00000001 ;; A comment containing 0 and 1\n00001111");
        let reader = unsafe { input.as_mut_vec() };
        let mut writer: Vec<u8> = Vec::new();

        process(&mut reader.as_slice(), &mut writer);

        assert_eq!(writer, vec![0x1, 0xf]);
    }
}
