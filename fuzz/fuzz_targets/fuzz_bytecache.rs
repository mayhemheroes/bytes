use honggfuzz::fuzz;

use std::collections::HashMap;
use std::io;
use serde_bytes::Bytes;

fn main() {
    loop {
        fuzz!(|data: &[u8]| {
            let mut cache = HashMap::new();
            let data1 = &data.to_owned();
            let data2 = &data.to_owned();
            let data3 = &data.to_owned();
            cache.insert(3, Bytes::new(&data1));
            cache.insert(2, Bytes::new(&data2));
            cache.insert(1, Bytes::new(&data3));
            let _ = bincode::serialize_into(&mut io::stdout(), &cache);
        });
    }
}