use exitfailure::ExitFailure;
use failure::ResultExt;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Cli {
    pattern: String,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<(), ExitFailure> {
    //let args = Cli::from_args();
    let path = "test.txt";
    let content = std::fs::read_to_string(path)
        .with_context(|_| format!("could not read file `{}`", path))?;

    println!("file content: {}", content);

    /*
    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
    */

    Ok(())
}
