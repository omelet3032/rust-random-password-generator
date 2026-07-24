use rand::prelude::*;

fn main() {

    let mut rng: ThreadRng = rand::rng();

    // let password: String = (0..16).map(|_| rng.random_range(33u8..=126u8) as char).collect();

    let char_range = 33u8 as char..=126u8 as char; 
    let mut vec_char:Vec<char> = Vec::new();

    for i in char_range {
        vec_char.push(i);
    }

    for (i, value) in vec_char.iter().enumerate() {
        println!("i : {}, value: {}", i, value);
    }
    


}

