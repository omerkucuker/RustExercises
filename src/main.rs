fn main() {

    //faktöriyel hesaplama
    faktöriyel();

    //dizi-vektör
    diziler();
}


fn faktöriyel (){
    let num = 4;
    let mut temp = num;
    let mut result=1;
    while temp>=1 {
        result *= temp;
        temp -=1;
    }
    println!("{} sayisinin faktoriyeli = {}",num,result);
}

fn diziler(){

    let isimler = vec!["ahmet","mehmet","ali"];

    for (sira,isim) in isimler.iter().enumerate(){
        print!("İsim: {} , sira: {} \n",isim,sira);
    }
}