mod arth;
use clap::{Arg, Command};

fn main() {
    let matches = Command::new("CLI Calculator")
        .version("1.0")
        .author("Arunkumar Turaka")
        .about("Performs basic arth operations")
        .arg(Arg::new("operation")
            .short('o')
            .long("operation")
            .value_name("OPERATION")
            .help("Specify the operation: add, +, sub, -, mul, *, div, /")
            .required(true)
            .num_args(1))
        .arg(Arg::new("operand1")
            .short('1')  // Use a single character for the short flag
            .long("operand1")
            .value_name("OPERAND1")
            .help("The first operand")
            .required(true)
            .num_args(1))
        .arg(Arg::new("operand2")
            .short('2')  // Use a single character for the short flag
            .long("operand2")
            .value_name("OPERAND2")
            .help("The second operand")
            .required(true)
            .num_args(1))
        .get_matches();

    if let Err(e) = run(matches) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run(matches: clap::ArgMatches) -> Result<(), String> {
    let operation = matches
        .get_one::<String>("operation")
        .ok_or("Operation not provided. Use --operation or -o to specify the operation")?;
    let operand1 = parse_operand(matches.get_one::<String>("operand1"))?;
    let operand2 = parse_operand(matches.get_one::<String>("operand2"))?;

    let result = match operation.as_str() {
        "add" | "+" => Ok(arth::add(operand1, operand2)),
        "sub" | "-" => Ok(arth::subtract(operand1, operand2)),
        "mul" | "*" => Ok(arth::multiply(operand1, operand2)),
        "div" | "/" => arth::divide(operand1, operand2),
        _ => Err("Invalid operation. Use add, sub, mul, or div.".to_string()),
    };

    match result {
        Ok(value) => {
            println!("Result: {}", value);
            Ok(())
        }
        Err(e) => Err(e),
    }
}

fn parse_operand(operand: Option<&String>) -> Result<f64, String> {
    operand
        .ok_or("Operand is missing".to_string())?
        .parse::<f64>()
        .map_err(|_| "Invalid number for operand".to_string())
}
