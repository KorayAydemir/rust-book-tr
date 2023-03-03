// ANCHOR: here
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(dosya) => dosya,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
// ANCHOR_END: here

fn main() {
    let kullanıcı_adı =
        read_username_from_file().expect("Kullanıcı adı alınamadı.");
}
