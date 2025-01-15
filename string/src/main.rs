fn main() {
    let some_sentence = String::from("Rust is a systems programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety.");
    println!("The first word is: {}", first_word(some_sentence));
}

fn first_word(s: String) -> String {
    let mut ans = String::from("");
    for c in s.chars() {
        ans.push_str(c.to_string().as_str());
        if c == ' ' {
            break;
        }
    }
    return ans;
}
