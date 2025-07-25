fn main() {
    // let input = include_str!("../input_test.txt");
    let input = include_str!("../input.txt");
    let result: i32 = input.split_whitespace().map(|line| {
        let mut dimensions: Vec<i32> = line.split('x').map(|x| x.parse::<i32>().unwrap()).collect();
        let wrapping_paper = 2 * dimensions[0] * dimensions[1] + 2 * dimensions[1] * dimensions[2] + 2 * dimensions[2] * dimensions[0];
        dimensions.sort();
        let slack = dimensions[0] * dimensions[1];
        wrapping_paper + slack
    }).sum();
    println!("{}", result);
}
