fn main() {
    let input = include_str!("../input.txt");
    let result: i32 = input.chars().map(|x| match x {'(' => 1, ')' => -1, _ => 0}).sum();
    println!("{}", result);
}
