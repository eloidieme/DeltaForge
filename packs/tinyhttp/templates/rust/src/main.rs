use std::env;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        eprintln!("usage: tinyhttp <command> ...");
        return ExitCode::FAILURE;
    }

    println!("TinyHTTP starter: implement command {:?}", args);
    ExitCode::SUCCESS
}
