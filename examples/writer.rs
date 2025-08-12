use embytes_buffer::{Buffer, BufferWriter, ReadWrite};
use embedded_io::{Read, Write};



fn main () {
    let mut bytes = [0; 1024];
    let mut buffer = Buffer::new(&mut bytes);

    // Write some bytes
    buffer.write_all("abc".as_bytes()).unwrap();

    // Create a writer, write some bytes but do not commit
    // writer implements DerefMut<Target = [u8]> and can be used as a mutable bytes slice
    let mut writer = buffer.create_writer();
    writer[0] = '$' as u8;

    // The writer is dropped without committing so the write has no effect
    drop(writer);

    // Create a new writer
    let mut writer = buffer.create_writer();
    writer[0] = 'd' as u8;
    writer[1] = 'e' as u8;
    writer[2] = 'f' as u8;

    // Commit that 3 bytes are written
    // writing bytes has only an effect if the written bytes are committed
    writer.commit(3).unwrap();
    drop(writer); // drop the writer to follow the borrowing rules. at drop the written bytes are committed

    let mut result = [0; 1024];
    let bytes_read = buffer.read(&mut result).unwrap();
    assert_eq!("abcdef".as_bytes(), &result[..bytes_read]);
}

