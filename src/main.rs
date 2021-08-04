fn main() {

    //faktöriyel hesaplama
    let num = 4;
    let mut temp = num;
    let mut result=1;
    while temp>=1 {
        result *= temp;
        temp -=1;
    }
    println!("{} sayisinin faktoriyeli = {}",num,result);

    
}
