// ANCHOR: here
use std::fs;
use std::io;

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
// ANCHOR_END: here

fn main() {
    let kullanıcı_adı =
        read_username_from_file().expect("Kullanıcı adı alınamadı.");
}
