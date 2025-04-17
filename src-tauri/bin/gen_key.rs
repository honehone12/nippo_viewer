use chacha20poly1305::{
    aead::OsRng, ChaCha20Poly1305, KeyInit
};
use base64::prelude::*;

fn main() {
    let key = ChaCha20Poly1305::generate_key(&mut OsRng);
    let key = key.as_slice();
    let b64_key = BASE64_STANDARD.encode(key);
    println!("\n{b64_key}\n");
}
