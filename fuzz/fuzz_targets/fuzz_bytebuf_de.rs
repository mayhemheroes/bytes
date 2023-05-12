use honggfuzz::fuzz;

use std::collections::HashMap;
use serde_bytes::ByteBuf;

fn deserialize_bytebufs(data: &[u8]) -> bincode::Result<()> {
    let map: HashMap<u32, ByteBuf> = bincode::deserialize(&data[..])?;
    println!("{:?}", map);
    Ok(())
}

fn main() {
    loop {
        fuzz!(|data: &[u8]| {
            let _ = deserialize_bytebufs(data).unwrap();
        });
    }
}