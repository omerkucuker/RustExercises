use std::str;

fn main() {

    let mut num : i32 = 4;

    //parametre olarak gönderilen sayıyı referansı üzerinden geri döndürme
    iki_katini_al(&mut num);
    print!("num'un yeni degeri: {}\n",num);

    //struct ornek
    let ogrenci1 : Ogrenci = Ogrenci{
        ad:String::from("Ahmet"),
        notlar: DersNotlari{
            ingilizce:60,
            matematik:70,
        }
    };

    print!("Ogrencinin adi {}, İngilizce dersinin notu :{} , Matematik dersinin notu: {}\n",
    ogrenci1.ad, ogrenci1.notlar.ingilizce, ogrenci1.notlar.matematik);


    //faktöriyel hesaplama
    faktoriyel(4);

    //dizi-vektör
    diziler();
}

struct DersNotlari {
    ingilizce : u8,
    matematik: u8,
}

struct Ogrenci{
    ad : String,
    notlar : DersNotlari,
}

fn iki_katini_al(number: &mut i32){

    *number = *number * 2;

}


fn faktoriyel (number:i32) {
    let mut temp = number;
    let mut result=1;
    while temp>=1 {
        result *= temp;
        temp -=1;
    }
    println!("{} sayisinin faktoriyeli = {}\n",number,result);
    
}

fn diziler(){

    let isimler = vec!["ahmet","mehmet","ali"];

    for (sira,isim) in isimler.iter().enumerate(){
        print!("İsim: {} , sira: {} \n",isim,sira);
    }
}