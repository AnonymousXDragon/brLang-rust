use std::{
    env,
    fs::File,
    io::{self, Read},
    process,
};

mod scan;
use scan::scanner;

fn main() -> io::Result<()> {
    let mut r#args = env::args();
    let mut file_data: String = String::new();

    if args.len() > 1 {
        let file_name = args.nth(1).unwrap();
        let mut buf = File::open(file_name).expect("file not found");
        buf.read_to_string(&mut file_data)
            .expect("unable to read the file");

        let s = scanner::Scanner::new(file_data.to_string());
        println!("{:?}", s.source);
    } else {
        eprintln!("exiting !");
        process::exit(1);
    }

    Ok(())
}
