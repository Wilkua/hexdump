use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Invalid command line arguments: Need filename");
        process::exit(1);
    }

    let filename = &args[1];
    let contents = fs::read(filename).unwrap_or_else(|err| {
        eprintln!("Failed to read from file {}: {}", args[1], err);
        process::exit(1);
    });

    print!("Offset  00  01  02  03  04  05  06  07  08  09  0A  0B  0C  0D  0E  0F");
    for offset in 0..contents.len() {
        if offset % 16 == 0 {
            print!("\n{:06x}  ", offset);
        }
        print!("{:02x}  ", contents[offset]);
    }
}
