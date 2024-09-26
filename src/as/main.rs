use std::env::args;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut incls = Vec::new();

    let mut opt = None;
    let mut files = Vec::new();
    for arg in args().skip(1) {
        match arg.as_str() {
            "-h" | "-?" => {
                usage();
                return Ok(());
            }

            "-I" => opt = Some("I"),
            "-D" => opt = Some("D"),

            _ => {
                if arg.starts_with('-') {
                    println!("Unknown option: {}", arg)
                }
            }
        }

        match opt {
            Some("I") => {
                opt = None;
                incls.push(arg);
                continue;
            }

            Some("D") => {
                opt = None;
                println!("Defining {}", arg);
                continue;
            }

            _ => (),
        }

        files.push(arg);
    }

    for file in files {
        assemble(&file)?;
    }

    Ok(())
}

fn usage() {
    println!("Usage: as8086 [-h|-?] [-I dir] [-D def] file ...");
}

fn assemble(_file: &str) -> Result<()> {
    // let mut f = File::open
    Ok(())
}
