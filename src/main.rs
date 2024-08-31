mod calc;
mod fibtype;
use fibtype::Fibonacci;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 3{
        println!("calculate, calc_debug")
    }
    if args.len() != 3{
        panic!("Must pass in exactly 2 arguments");
    }
    match &*(args[1]) {
        "calculate" => {
            let fib: Fibonacci = Fibonacci::new(
                args[2].parse::<usize>().unwrap()
            );
            println!("{fib}");
        }
        "calc_debug" => {
            let fib: Fibonacci = Fibonacci::new(
                args[2].parse::<usize>().unwrap()
            );
            println!("{:?}", fib);
        }
        _ => {panic!("Invalid command")}
    }
}
