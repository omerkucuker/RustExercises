fn main() {

    //faktöriyel hesaplama
    faktöriyel(4);

    //dizi-vektör
    diziler();
}


fn faktöriyel (number:i32) {
    let mut temp = number;
    let mut result=1;
    while temp>=1 {
        result *= temp;
        temp -=1;
    }
    println!("{} sayisinin faktoriyeli = {}",number,result);
    
}

fn diziler(){

    let isimler = vec!["ahmet","mehmet","ali"];

    for (sira,isim) in isimler.iter().enumerate(){
        print!("İsim: {} , sira: {} \n",isim,sira);
    }
}