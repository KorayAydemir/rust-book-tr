use std::fs::File;

fn main() {
    let f =
        File::open("hello.txt").expect("hello.txt 'yi açmak başarısız oldu");
}
