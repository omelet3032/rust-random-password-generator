use rand::prelude::*;

fn main() {

    // let password: String = (0..16).map(|_| rng.random_range(33u8..=126u8) as char).collect();

    let char_range = 33u8 as char..=126u8 as char;
    let mut vec_char: Vec<char> = Vec::new();
    let password = String::new();

    for i in char_range {
        vec_char.push(i);
    }

    let length: usize = 16;
    
    let result = generate_password(length);
    println!("result : {} ", result);
}

fn generate_password(length:usize) -> String {

    let mut rng: ThreadRng = rand::rng();

    let vec_char: Vec<char> = (33u8..=126u8).map(|c| c as char).collect();

    // let mut password = String::new();
    
    let password: String = (0..length).map(|_| {
        let idx = rng.random_range(0..vec_char.len());
        vec_char[idx]
    }).collect();

    password


    // for _ in 0..16 {
    //     let idx = rng.random_range(0..vec_char.len());
    //     password.push(vec_char[idx]);
    // }

}