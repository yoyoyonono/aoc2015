fn main() {
    let input = include_str!("../input.txt");
    let transitions = input.chars().map(|x| match x {'(' => 1, ')' => -1, _ => 0});
    let mut floor = 0;
    let mut found_index = 0;
    for (index, transition) in transitions.enumerate() {
        floor += transition;
        if floor < 0 {
            found_index = index;
            break;
        }
    }
    println!("{}", found_index + 1);
}
