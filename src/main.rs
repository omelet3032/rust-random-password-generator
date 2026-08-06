use rand::prelude::*;
use rand::distr::Uniform;

fn main() {

    let length: usize = 16;
    
    let result = generate_password(length);
    println!("result : {} ", result);
}

fn generate_password(length:usize) -> String {

    let mut rng = rand::rng(); 

    let dist = Uniform::new_inclusive(33u8, 126u8).unwrap();
    let password: String = (0..length).map(|_| rng.sample(dist) as char).collect();
    // let password: String = (0..length).map(|_| rng.random_range((33u8..=126u8)) as char).collect();

    password
}

// 
fn generate_password_only_number(length:usize) -> String {
    
    let mut rng = rand::rng();
    // let dist = Uniform::new_inclusive(48u8, 57u8).unwrap();
    let dist = Uniform::new_inclusive(b'0', b'9').unwrap();

    let password:String = (0..length).map(|_| rng.sample(dist) as char).collect();

    password

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_empty_string_when_length_is_zero() {

        let length: usize = 0;

        let result = generate_password(length);
        
        assert_eq!(result, "");

    }

    #[test]
    fn return_only_number() {
        let length: usize = 8;

        let result = generate_password_only_number(length);

        println!("result : {}", result);



    }

    
    
}
