use std::io::{self, Read};

fn main() {
    // n'inci sayının fibonacci'si için
    // 1 den başlayarak 1 + 1 = 2, 2 + 1 = 3, 3 + 2 = 5, 5 + 3 = 8 şeklinde ilerliyor
    //yani F(n) =  F(n-1) + F(n)
    // bu durumda a ve b = 1 olursa, a + b = sonuc, a = b, b = sonuc, a + b = sonuc
    // ikisi de 1 den başlarsa a(1) + b(1) = 2, a(1) = b(1) = b(1) = 2
    // a(1) + b(2) = 3, a(1) = b(2), b(2) = 3
    // a(2) + b(3) = 5, a(2) = b(3), b(3) = 5

    let mut a: i32 = 1;
    let mut b: i32 = 1;
    let mut n = String::new();
    let mut i = 0;
    let mut add = 0;

    println!("Lütfen Fibonacci Eleman Sayısını Giriniz: ");
    io::stdin().read_line(&mut n).expect("İşlem Sırası Hata");
    let n: i32 = match n.trim().parse() {
        Ok(num) => num,
        Err(_) => 5,
    };
    println!("{}. elemanın fibonacci dizisi:", n);

    while i < n{
        a = b;
        b = add;
        add = a + b;
        println!("{}", add);
        i += 1;
    }
}