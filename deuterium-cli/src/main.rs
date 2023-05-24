use std::fs::File;
use std::io::Read;
use bincode::deserialize;

use deuterium::LuaHeader;

fn main() {
    // Read the binary data from a file
    let mut file = File::open("bytecode.bin").expect("Failed to open file");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Failed to read file");

    // Deserialize the LuaHeader struct from the binary data
    let header: LuaHeader = deserialize(&buffer).expect("Failed to deserialize");

    // Print the decoded header
    println!("{:#?}", header);
}
