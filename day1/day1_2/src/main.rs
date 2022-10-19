fn count_letter(s: &str, c: char) -> usize {
    let mut count = 0;
    for _c in s.chars() {
        if _c.to_ascii_uppercase() == c.to_ascii_uppercase() {
            count += 1;
        }
    }

    return count;
}
fn main() {
    let _string = "Hello Monday, Today is a wonderful day";
    let mut line:String = String::new();
    println!("Please input a character from keyboard:");
    std::io::stdin().read_line(&mut line).unwrap();
    line.pop();
    let to_count_letter = line.chars().next().unwrap();
    println!("You input '{}' from keyboard", to_count_letter);
    let count = count_letter(&_string, to_count_letter);

    println!("There are {} letter '{}' in \"{}\"", count, to_count_letter, &_string);
}
