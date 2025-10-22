use ::std::io::{self, Read, Write};

const CHUNK_SIZE: usize = 16 * 1024;

fn main() {
    let mut buffer = [0; CHUNK_SIZE];
    let num_read = io::stdin().read(&mut buffer).unwrap();
    eprintln!("Number of Bytes read{}", num_read);
}
