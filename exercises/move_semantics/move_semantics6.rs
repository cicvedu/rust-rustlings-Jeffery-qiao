// move_semantics6.rs
//
// You can't change anything except adding or removing references.
//
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.


fn main() {
    let data = "Rust is great!".to_string();

    let last_char = get_char(&data);
    println!("Last character: {}", last_char);

    let uppercase_data = string_uppercase(&data);
    println!("Uppercase data: {}", uppercase_data);
}

// Does not take ownership
fn get_char(data: &str) -> char {
    data.chars().last().unwrap()
}

// Takes ownership
fn string_uppercase(data: &String) -> String {
    data.to_uppercase()
}