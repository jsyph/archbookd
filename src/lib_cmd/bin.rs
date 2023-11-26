use nix::unistd::Uid;
use std::env;

#[tokio::main]
async fn main() {
    // ensure root
    if !Uid::effective().is_root() {
        println!("Must run as root");

        if cfg!(not(debug_assertions)) {
            return;
        }

        println!("Debug mode enabled");
    }

    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    if args.len() <= 1 {
        println!("Invalid number of arguments");
        return;
    }
}
