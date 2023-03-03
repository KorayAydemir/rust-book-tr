use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Numarayı tahmin et!");

    let gizli_numara = rand::thread_rng().gen_range(1..101);

    // ANCHOR: here
    loop {
        // --snip--

        // ANCHOR_END: here
        println!("Lütfen tahmininizi girin.");

        let mut tahmin = String::new();

        io::stdin()
            .read_line(&mut tahmin)
            .expect("Satırı okumak başarısız oldu.");

        // ANCHOR: here
        let tahmin: i32 = match tahmin.trim().parse() {
            Ok(sayı) => sayı,
            Err(_) => continue,
        };

        if tahmin < 1 || tahmin > 100 {
            println!("Gizli numara 1 ile 100 arasında olacaktır.");
            continue;
        }

        match tahmin.cmp(&gizli_numara) {
            // --snip--
            // ANCHOR_END: here
            Ordering::Less => println!("Fazla küçük!"),
            Ordering::Greater => println!("Fazla büyük!"),
            Ordering::Equal => {
                println!("Kazandın!");
                break;
            }
        }
        // ANCHOR: here
    }
    // ANCHOR_END: here
}
