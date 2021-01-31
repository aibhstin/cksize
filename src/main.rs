use std::env;
use std::fs;
use std::process;

fn main() {
    let b_c = 1000;
    let kb_c = 100_000;
    let mb_c = 100_000_000;
    let gb_c = 100_000_000_000;
    let tb_c = 100_000_000_000_000;

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Specify filepath(s)");
        process::exit(1);
    }

    for arg in args.iter().skip(1) {
        let metadata = fs::metadata(arg).expect("Please enter a valid filepath");
        let filesize = metadata.len();

        let (suffix, updated_filesize) = if filesize < b_c {
            (" bytes", filesize)
        } else if filesize < kb_c {
            (" kilobytes", filesize / 1000)
        } else if filesize < mb_c {
            (" megabytes", filesize / 1000_000)
        } else if filesize < gb_c {
            (" gigabytes", filesize / 1000_000_000)
        } else if filesize < tb_c {
            (" terabytes",filesize / 1000_000_000_000)
        } else {
            (" bytes", filesize)
        };

        println!("[{}]: {}{}", arg, updated_filesize, suffix);
    }
}
