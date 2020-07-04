use exitfailure::ExitFailure;
use failure::ResultExt;
use std::io::{self, Write};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Cli {
    pattern: String,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn find_matches(content: &str, pattern: &str) {
    for line in content.lines() {
        if line.contains(pattern) {
            println!("{}", line);
        }
    }
}

fn main() -> Result<(), ExitFailure> {
    let args = Cli::from_args();
    let path = &args.path;
    let content = std::fs::read_to_string(path)
        .with_context(|_| format!("could not read file `{:?}`", path))?;

    find_matches(&content, &args.pattern);

    Ok(())
}
