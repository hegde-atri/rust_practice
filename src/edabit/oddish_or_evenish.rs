pub fn oddish_or_evenish(input: u32) {
    if input
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .sum::<u32>()
        % 2
        == 0
    {
        println!("evenish")
    } else {
        println!("oddish")
    }
}
