fn main() {
    day1()
}

fn day1() {
    let content = std::fs::read_to_string("input/1.txt").unwrap();
    let elves = content.split("\n\n");
    
    let mut calorie_sums : Vec<i32> = elves.map(|a| a.split("\n")
    .map(|a| a.parse::<i32>().unwrap()) // convert to int
    .sum()).collect();
    
    calorie_sums.sort();
    println!("{:?}", calorie_sums[calorie_sums.len()-1]);
    println!("{:?}", calorie_sums[calorie_sums.len()-3..].iter().sum::<i32>());
}
