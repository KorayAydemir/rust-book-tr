use rand::Rng;
use std::cmp::Ordering;
use std::io;

// ANCHOR: here
pub struct Tahmin {
    değer: i32,
}

impl Tahmin {
    pub fn yeni(değer: i32) -> Tahmin {
        if değer < 1 || değer > 100 {
            panic!(
                "Tahmin değeri 1 ile 100 arasında olmalıdır, alınan: {}.",
                değer
            );
        }

        Tahmin { değer }
    }

    pub fn değer(&self) -> i32 {
        self.değer
    }
}
// ANCHOR_END: here

fn main() {
    println!("Sayıyı tahmin et!");

    let gizli_numara = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Lütfen tahmininizi girin.");

        let mut tahmin = String::new();

        io::stdin()
            .read_line(&mut tahmin)
            .expect("Satırı okuma başarısız oldu.");

        let tahmin: i32 = match tahmin.trim().parse() {
            Ok(sayı) => sayı,
            Err(_) => continue,
        };

        let tahmin = Tahmin::yeni(tahmin);

        match tahmin.değer().cmp(&gizli_numara) {
            Ordering::Less => println!("Fazla küçük!"),
            Ordering::Greater => println!("Fazla büyük!"),
            Ordering::Equal => {
                println!("Kazandın!");
                break;
            }
        }
    }
}

