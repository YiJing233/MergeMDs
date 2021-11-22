use structopt::StructOpt;
use std::fs::OpenOptions;
use std::io::Write;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}
fn main() {
    let args = Cli::from_args();
    println!(
        "Your args are: pattern: {}, path: {:?}",
        args.pattern, args.path
    );
    let file_names = std::fs::read_dir(args.path).unwrap();
    
    std::fs::File::create(".\\output.md").expect("create failed");
    let mut file = OpenOptions::new().append(true).open(".\\output.md").expect(
      "cannot open file");

    for file_name in file_names {
        let content =
            std::fs::read_to_string(&file_name.unwrap().path()).expect("Read file failed");
        println!("content: : {}", content);
        file.write_all(("\n".to_string() + &content).as_bytes()).expect("Write error");
    }
}
