mod day1;
mod day2;

fn run_day(stdout: &mut dyn std::io::Write, day : &str) {
    let content = std::fs::read_to_string(format!("input/{day}.txt")).unwrap();
    match day {
        "1" => day1::run(stdout, &content),
        "2" => day2::run(stdout, &content),
        _ => {}
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut day = "1";
    if args.len() > 1 { day = &args[1]; }
    run_day(&mut std::io::stdout(), day);
}

#[test]
fn test1() {
    let mut stdout = Vec::new();
    run_day(&mut stdout, "1");
    assert_eq!(stdout, b"70698\n206643\n");
}

#[test]
fn test2() {
    let mut stdout = Vec::new();
    run_day(&mut stdout, "2");
    assert_eq!(stdout, b"11873\n12014\n");
}

