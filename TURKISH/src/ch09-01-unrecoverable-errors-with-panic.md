## `panic!` ile Kurtarılamaz Hatalar

Bazen, kodunuzda kötü şeyler yaşanır, ve bunun hakkında yapabilecek hiçbir şeyiniz
yoktur. Bu durumlarda, Rust `panic!` makrosuna sahiptir. `panic!` makrosu çalıştığında,
programınız bir arıza mesajı yazdıracak, çözüp yığıtı temizleyecek, ve sonra çıkış
yapacaktır. Çoğunlukla bir bug veya benzeri bir şey tespit edildiğinde ve programı
yazdığımız anda sorunu nasıl işleyeceğimiz belli olmadığında bir panik uyandırmış oluruz.

> ### Yığını Çözmek veya Bir Paniğe Yanıt Olarak Yarıda Kesmek.
>
> Varsayılan olarak, bir panik meydana geldiğinde, program *çözülmeye* başlar, bu
>demektir ki; Rust yığıttan yukarı geri yürüyüp çıkar ve veriyi karşılaştığı her fonksiyondan
>temizler. Ancak, bu geri yürüme ve temizleme çok fazla iştir. Bu yüzden Rust,
>anında *yarıda kesmeye* alternatif seçmenize izin verir, ki bu da programı temizlemeden
>sonlandırır. Programın kullandığı hafızanın sonra işletim sistemi tarafından temizlenmesi
>gerekecektir. Eğer projenizden çıkan bilgisayar tarafından yorumlanan dosyayı olabildiğince
>küçültmeniz gerekiyor ise, *Cargo.toml* dosyanızdaki uygun `[profile]` kısımlarına
>`panic = 'abort'` ekleyerek, bir panik olduğunda çözme yerine yarıda kesmeye
>değiştirebilirsiniz.
>
> ```toml
> [profile.release]
> panic = 'abort'
> ```

Basit bir program içinde `panic!` çağırmayı deneyelim:

<span class="filename">Dosya adı: src/main.rs</span>

```rust,should_panic,panics
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-01-panic/src/main.rs}}
```

`panic!` çağrısı son iki satırda bulunan hata mesajına sebep olur. İlk satır panik
mesajımızı ve kaynak kodumuzda paniğin meydana geldiği yeri gösterir: *src/main.rs:2:5*
hatanın *src/main.rs* dosyamızın ikinci satırda, beşinci harfinde olduğunu belirtir.

Bu durumda, işaret edilen satır kodumuzun bir parçası, ve eğer bu satıra gidersek,
`panic!` makro çağrısını görürüz. Diğer durumlarda, `panic!` çağrısı kodumuzun
çağırdığı bir kodun içinde olabilir, ve hata mesajı tarafından bildirilen dosya
adı ve satır numarası bizim kodumuzun `panic!` çağrısına neden olan satiri değil,
başka birisinin kodunda `panic!` makrosunun çağrıldığı yer olacaktır. Soruna sebep
olan yeri bulmak için `panic!` çağrısının geldiği fonksiyonların geri izlemesini
kullanabiliriz. Geri izlemeleri sonra daha detaylı tartışacağız.

### Bir `panic!` Geri İzlemesini Kullanmak

`panic!` çağrısının direkt bizim kodumuz tarafından çağrılması yerine, kodumuzdaki
bir bug yüzünden çağrının bir kütüphaneden gelmesinin nasıl olduğunu görmek
için haydi başka bir örneğe bakalım. Listeleme 9-1'de, bir vektörün geçerli
indekslerinin ötesindeki bir indekse erişmeye çalışan bir kod vardır.

<span class="filename">Dosya adı: src/main.rs</span>

```rust,should_panic,panics
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-01/src/main.rs}}
```

<span class="caption">Listeleme 9-1: Bir vektörün sonunun ötesindeki bir elemana
erişmeye çalışmak bir `panic!` çağrısına neden olacaktır.</span>

Burada, vektörümüzün 100'üncü elementine erişmeye çalışıyoruz (bu indeks 99
demek oluyor çünkü indeksleme sıfırdan başlar), ama vektörün sadece 3 elemanı var.
Bu durumda, Rust panikleyecektir. `[]` kullanmak normalde bir eleman döndürür,
ama eğer geçersiz bir indeks girerseniz, Rust'ın geri döndürmesinin doğru olacağı
hiçbir eleman yoktur.

C'de, bir data yapısının sonunun ötesini okumaya çalışmak tanımsız davranıştır.
Hafıza o veri yapısına ait olmasa bile, veri yapısındaki o elemanın olacağı konuma
karşılık gelen yerde hafızada ne varsa onu elde edersiniz. Buna *arabellek aşırı
okuması* denir ve eğer bir saldırgan bu veri yapısının ötesinde depolanan, okumamaları
gereken verileri okumalarını sağlayacak şekilde indeksi manipüle edebilirse güvenlik
açıklarına yol açabilir.

Eğer var olmayan indeksteki bir elemanı okumaya çalışırsanız, Rust programınızı bu
tür bir açıktan korumak için uygulamanın çalışmasını durduracaktır ve devam etmeyi
reddedecektir. Haydi deneyip görelim:

```console
{{#include ../listings/ch09-error-handling/listing-09-01/output.txt}}
```

Bu hata `main.rs`'imizdeki, indeks 99 a erişmeye çalıştığımız satır 4'ü işaret eder.
Sonraki not satırı, bize tam olarak bu hataya tam olarak neyin sebep olduğu olduğu hakkında
bir geri izleme almak için `RUST_BACKTRACE` ortam değişkenini belirleyebileceğimizi söyler.
*Geri izleme*, o ana kadar çağırılmış bütün fonksiyonların bir listesidir. Rust'ta
geri izlemeler diğer dillerde oldukları gibi çalışır, geri izlemeyi okumanın anahtarı en
yukarıdan başlamak ve kendi yazdığınız dosyaları görene kadar okumaktır. Sorunun kaynağı
bu noktadır. Bu noktanın üzerindeki satırlar sizin çağırdığınız kod; altındaki satırlar
sizin kodunuzu çağıran koddur. Bu önce-ve-sonra satırları öz Rust kodu, standart kütüphane
kodu, veya kullandığınız sandıkları içerebilir. Haydi `RUST_BACKTRACE` ortam değişkenini
0 hariç herhangi bir değere ayarlayıp bir geri izleme almaya çalışalım. Listeleme 9-2
sizin göreceğinize benzer bir çıktıyı gösterir.

<!-- manual-regeneration
cd listings/ch09-error-handling/listing-09-01
RUST_BACKTRACE=1 cargo run
copy the backtrace output below
check the backtrace number mentioned in the text below the listing
-->
```console
$ RUST_BACKTRACE=1 cargo run
thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', src/main.rs:4:5
stack backtrace:
   0: rust_begin_unwind
             at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/std/src/panicking.rs:483
   1: core::panicking::panic_fmt
             at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/core/src/panicking.rs:85
   2: core::panicking::panic_bounds_check
             at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/core/src/panicking.rs:62
   3: <usize as core::slice::index::SliceIndex<[T]>>::index
             at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/core/src/slice/index.rs:255
   4: core::slice::index::<impl core::ops::index::Index<I> for [T]>::index
             at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/core/src/slice/index.rs:15
   5: <alloc::vec::Vec<T> as core::ops::index::Index<I>>::index
             at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/alloc/src/vec.rs:1982
   6: panic::main
             at ./src/main.rs:4
   7: core::ops::function::FnOnce::call_once
             at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/core/src/ops/function.rs:227
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

<span class="caption">Listeleme 9-2: Ortam değişkeni `RUST_BACKTRACE` ayarlanmış iken, `panic!`e bir
çağrı tarafından oluşturulan geri izleme.</span>

Oldukça uzun bir çıktı! Sizin gördüğünüz tam çıktı işletim sisteminize ve Rust
versiyonunuza göre biraz farklı olabilir. Bu bilgileri içeren geri
izlemeyi alabilmek için, hata ayıklama sembolleriniz etkinleştirilmiş olmak
zorundadır. Burada yaptığımız gibi, `--release` bayrağı olmadan `cargo build`
veya `cargo run` kullanırken hata sembolleri varsayılan olarak etkindir.

Listeleme 9-2'deki çıktıda, geri izlemenin 6. satırı projemizdeki sorun
çıkaran satırı işaret ediyor; *src/main.rs*'in 4'üncü satırı. Eğer programımızın
paniklemesini istemiyorsak, araştırmamıza bizim yazdığımız bir dosyadan bahseden
ilk satırdan başlamalıyız. Kasten panikleyecek kod yazdığımız Listeleme 9-1'de,
paniği düzeltmenin yolu; vektörün indeks aralığının ötesinde bir elemanı talep
etmemektir. İleride kodunuz paniklediğinde, kodun paniğe sebep olacak kadar hangi
değerlerle ne yaptığını ve onun yerine ne yapması gerektiğini anlamanız gerecektir.

`panic!`'e ve ne zaman hata koşullarını işlemek için `panic!` kullanmamamız
gerektiğine bu bölümün daha sonrasında [“`panic!`lemek ya da
`panic!`lememek”][to-panic-or-not-to-panic] kısmında geri döneceğiz. Daha sonra,
`Result` kullanarak bir hatadan nasıl kurtarabileceğimize bakacağız.

[to-panic-or-not-to-panic]:
ch09-03-to-panic-or-not-to-panic.html#to-panic-or-not-to-panic




