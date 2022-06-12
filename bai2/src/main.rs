use std::ops::Index;

fn main() {
    let mut url =String::from("https://ars.els-cdn.com/content/image/1-s2.0-S0960982203005347-mmc6.Txtx");
    let mut input = String::from("t");
    let mut sum = 0;
    for character_url in url.chars() {
        for character_input in input.chars(){
            if character_input == character_url {
                sum = sum +1;
            }

        }
    }
    println!("{}",sum);
        

}

// PS : chỉ đúng trong bài toán này là input có 1 chữ số , em chưa nghĩ ra cách hay hơn :V