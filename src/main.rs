use exa::Exa;
use std::env::args_os;
use std::ffi::OsString;
use std::fs::File;
use std::io::{stdout, BufRead, BufReader, Write};
use std::iter::once;
use std::path::PathBuf;
use std::process::exit;

fn main() {
    let args: Vec<OsString> = args_os().skip(1).collect();
    let max = args.len();

    let mut options = Vec::with_capacity(max);
    let mut paths = Vec::with_capacity(max);

    for arg in args {
        if arg.to_string_lossy().starts_with('-') {
            options.push(arg);
        } else {
            paths.push(arg);
        }
    }

    let options = options.iter();

    if paths.is_empty() {
        paths.push(".".into());
    }

    let mut code = 0;
    let last = paths.len() - 1;
    for (i, p) in paths.into_iter().enumerate() {
        let path = PathBuf::from(p.clone());
        let name = path.display();

        if path.is_file() {
            let file = match File::open(&path) {
                Err(err) => {
                    eprintln!("failed to open file: {}\n{}", name, err);
                    continue;
                }
                Ok(f) => f,
            };

            if last != 0 {
                println!("=> {} <=", name);
            }

            let mut buffered = BufReader::new(file);
            loop {
                let mut line = Vec::new();
                if buffered
                    .read_until(0xA, &mut line)
                    .expect("error reading file")
                    > 0
                {
                    stdout().write(&line).expect("error writing");
                } else {
                    break;
                }
            }
        } else {
            let opts = options.clone().chain(once(&p));
            match Exa::from_args(opts, &mut stdout()) {
                Err(err) => {
                    eprintln!("{}", err);
                    exit(1);
                }
                Ok(mut exa) => {
                    if last != 0 {
                        println!("=> {} <=", name);
                    }

                    match exa.run().expect("exa bugged out?") {
                        0 => {}
                        n => {
                            code = n;
                        }
                    }
                }
            }
        }

        if i != last {
            println!();
        }
    }

    exit(code);
}
