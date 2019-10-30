
use structopt::StructOpt;
use std::io::{stdout, Write};
use std::path::PathBuf;

#[derive(StructOpt)]
struct Opt {
    #[structopt(parse(from_os_str))]
    commands: Vec<PathBuf>
}

fn main() {
    let stdout = stdout();
    let mut stdout = stdout.lock();
    let opts = Opt::from_args();

    let commands = opts.commands;
    let paths = std::env::var("PATH").unwrap();

    for command in &commands {
        let mut found_path: Option<PathBuf> = None;
        for mut path in std::env::split_paths(&paths) {
            path.push(command);
            if path.exists() {
                found_path = Some(path);
                break;
            }
        }

        let _ = match found_path {
            Some(path) => writeln!(stdout, "{}", path.display()),
            _ => writeln!(stdout, "{} not found", command.display())
        };
    }

    std::process::exit(0);
}
