// ANCHOR: here
use std::fs::File;
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}
// ANCHOR_END: here

fn main() {
    let kullanıcı_adı =
        read_username_from_file().expect("Kullanıcı adı alınamadı.");
}
