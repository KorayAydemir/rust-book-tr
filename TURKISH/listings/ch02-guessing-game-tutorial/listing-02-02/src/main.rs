use std::io;

fn main() {
    println!("Tuttuğum sayıyı tahmin edin!");

    println!("Tahmininizi girin.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Veri okuma hatası!");

    println!("Tahmininiz: {}", guess);
}
