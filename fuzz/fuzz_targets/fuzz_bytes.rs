use honggfuzz::fuzz;

fn main() {
    loop {
        fuzz!(|data: &[u8]| {
            let _ = serde_bytes::Bytes::new(&data);
        });
    }
}