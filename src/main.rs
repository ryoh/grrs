#[derive(Debug)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() {
    let pattern = std::env::args().nth(1).expect("no pattern give");
    let path = std::env::args().nth(2).expect("no path give");
    let args = Cli {
        pattern: pattern,
        path: std::path::PathBuf::from(path),
    };

    println!("{:#?}", args);
}
