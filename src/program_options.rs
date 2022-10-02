const HELP_MESSAGE: &str = "Usage:\ncargo run --release -- x y\nx - number of human players (optional, default = 1)\ny - AI search depth (optional, default = 6)";
pub static mut NUMBER_OF_PLAYERS: i32 = 1;
pub static mut DEPTH: i32 = 6;

fn print_help() {
    println!("{}", HELP_MESSAGE);
    std::process::exit(0);
}

pub fn program_options(args: Vec<String>) {
    if args.len() >= 2 {
        if let Ok(x) = args[1].parse::<i32>() {
            unsafe {
                NUMBER_OF_PLAYERS = x;
            }
        } else {
            print_help();
        }
    }
    if args.len() >= 3 {
        if let Ok(x) = args[2].parse::<i32>() {
            unsafe {
                DEPTH = x;
            }
        } else {
            print_help();
        }
    }
    if args.len() >= 4 {
        print_help();
    }
}
