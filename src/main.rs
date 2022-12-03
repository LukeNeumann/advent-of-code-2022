mod day1;
mod day2;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut day = "1";
    if args.len() > 1 { day = &args[1]; }

    let content = std::fs::read_to_string(format!("input/{day}.txt")).unwrap();
    match day {
        "1" => day1::run(&content),
        "2" => day2::run(&content),
        _ => {}
    }
}



