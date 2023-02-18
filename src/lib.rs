mod generator;

pub fn prin_random_number() {
    let n = generator::gen_ran();
    println!("Random u8 {}", n);
}
