use std::io::Write;

use embytes_buffer::{Buffer, ReadWrite};

fn main () {

    // In environments with std support you can use a vec for a head allocated buffer
    let bytes = vec![0u8; 1024];
    let mut buffer = Buffer::new(bytes);

    buffer.write_all(&[ 1, 2, 3, 4 ]).unwrap();
    let reader = buffer.create_reader();
    assert_eq!(&reader[..], &[ 1, 2, 3, 4  ]);

    // You can also use a mutable reference to the Vec
    let mut bytes = vec![0u8; 1024];
    let mut buffer = Buffer::new(&mut bytes[..]);

    buffer.write_all(&[ 1, 2, 3, 4 ]).unwrap();
    let reader = buffer.create_reader();
    assert_eq!(&reader[..], &[ 1, 2, 3, 4  ]);

    // BUT: The Buffer does not change the len of the vec
    // You cannot write to this buffer!
    let mut buffer = Buffer::new(Vec::new());
    let result = buffer.write_all(&[ 1, 2, 3, 4 ]);
    assert!(result.is_err());

}