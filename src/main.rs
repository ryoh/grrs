use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Cli {
    pattern: String,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::from_args();
    let result = std::fs::read_to_string(&args.path);
    match result {
        Ok(content) => {
            println!("File content: {}", content);
        }
        Err(error) => {
            eprint!("Oh noes: {}", error);
        }
    }

    /*
    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
    */
}
