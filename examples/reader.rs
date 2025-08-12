use core::str::from_utf8;
use std::io::Write;

use embytes_buffer::{Buffer, BufferReader, ReadWrite};



fn main () {
    let mut bytes = [0; 1024];
    let mut buffer = Buffer::new(&mut bytes);

    // Write some bytes
    buffer.write_all("👌abc".as_bytes()).unwrap();

    // try to read to a komma but there is none
    let reader = buffer.create_reader();
    let result = read_til_komma(&reader);
    assert_eq!(result, None);
    drop(reader);

    // Write a string that now contains a comma
    buffer.write_all("def,1234".as_bytes()).unwrap();

    // try to read to a komma. now there is one
    let reader = buffer.create_reader();
    let result = read_til_komma(&reader);
    assert_eq!(result, Some("👌abcdef"));
    drop(reader);

    assert_eq!(buffer.data(), "1234".as_bytes());
}

/// This method reads a string from buf until there is a comma
/// 
/// Returns:
/// - [`Option::None`] if the string is not complete yet
/// - [`Option::Some`] with the string if there is a string
fn read_til_komma<'a>(reader: &'a impl BufferReader) -> Option<&'a str> {

    if reader.is_empty() {
        return None;
    }

    let str = from_utf8(&reader)
        .expect("expected valid utf8");

    // Find the position of the first comma
    let comma_position = str.find(',');

    if let Some(comma_position) = comma_position {
        let data = &str[..comma_position];

        let bytes_read = data.as_bytes().len() + 1;

        // Tell the reader that you have read `data.len() + 1` bytes
        reader.add_bytes_read(bytes_read);
        Some(data)
    } else {
        None
    }
}