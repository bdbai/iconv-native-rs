use std::{
    fs::File,
    io::{stdin, stdout, Read, Write},
    path::PathBuf,
};

use iconv_native::ConvertLossyError;

struct Args {
    from: String,
    to: String,
    file_path: Option<PathBuf>,
}

fn parse_args() -> Result<Args, lexopt::Error> {
    use lexopt::prelude::*;

    let mut from = None;
    let mut to = None;
    let mut file_path = None;

    let mut parser = lexopt::Parser::from_env();
    while let Some(arg) = parser.next()? {
        match arg {
            Short('f') | Long("from") => {
                from = Some(parser.value()?.to_string_lossy().into_owned());
            }
            Short('t') | Long("to") => {
                to = Some(parser.value()?.to_string_lossy().into_owned());
            }
            Long("help") | Long("usage") => {
                println!("Usage: mini_iconv -f FROM -t TO FILE");
                std::process::exit(0);
            }
            Value(file) => {
                file_path = Some(file.into());
            }
            _ => return Err(arg.unexpected()),
        }
    }

    let from = from.ok_or("missing argument FROM")?;
    let to = to.ok_or("missing argument TO")?;

    Ok(Args {
        from,
        to,
        file_path,
    })
}

fn main() {
    let args = match parse_args() {
        Ok(args) => args,
        Err(err) => {
            eprintln!("Invalid args: {}", err);
            std::process::exit(1);
        }
    };

    let mut buf = vec![];
    let read_res = match args.file_path {
        Some(file_path) => File::open(file_path).and_then(|mut file| file.read_to_end(&mut buf)),
        None => stdin().read_to_end(&mut buf),
    };
    if let Err(err) = read_res {
        eprintln!("Failed to read input: {}", err);
        std::process::exit(2);
    }
    let res = iconv_native::convert_lossy(&buf, &args.from, &args.to);
    match res {
        Ok(res) => {
            stdout().write_all(&res).unwrap();
        }
        Err(ConvertLossyError::UnknownConversion) => {
            eprintln!("Unknown encoding");
            std::process::exit(3);
        }
    }
}
