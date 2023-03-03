use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(dosya) => dosya,
        Err(hata) => match hata.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Dosyayı oluştururken problem: {:?}", e),
            },
            diğer_hata => {
                panic!("Dosyayı oluştururken problem: {:?}", diğer_hata)
            }
        },
    };
}
