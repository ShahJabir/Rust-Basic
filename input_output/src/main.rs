fn string() {
    let s = "Assalamu Alaikum!";

    let index_0 = s.chars().nth(0);

    match index_0 {
        Some(c) => println!("{}", c),
        None => println!("Character is not found"),
    }
}




fn main() {
    let x8: i8 = -128;
    let y8: u8 = 255;
    let z8: f32 = 12.21;
    println!("Hello, world!");
    print!("This is a basic variable printing program => ");
    println!("x8: {},  y8: {}, z8: {}", x8, y8, z8);
    string();
}
