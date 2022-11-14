pub fn oddish_or_evenish(input: i32) {
    for char in input.to_string().chars() {
        let digit = char as i32;
        println!("{}", digit);
    }
}
