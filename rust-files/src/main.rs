use std::fs::File;
use std::io::Error;
use std::io::{BufRead, BufReader, ErrorKind};
use std::{env, fs};

const ALL: u32 = 1;
const LINE: u32 = 2;

fn read_all_file(path: &String) -> Result<String, Error> {
    fs::read_to_string(path)
}

fn read_line_file(path: &String, line: u32) -> Result<String, Error> {
    let f = File::open(path);
    let f = match f {
        Ok(f) => f,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => {
                _ = File::create(path);
                println!("No existía, se ha creado el fichero");
                return Result::Err(e);
            }
            _ => return Result::Err(Error::new(ErrorKind::Other, "Error irrecuperable")),
        },
    };

    let buff = BufReader::new(f);
    let mut a = 0;
    for i in buff.lines() {
        if a == line {
            return Result::Ok(i.unwrap());
        }
        a += 1;
    }
    return Result::Err(Error::new(ErrorKind::Other, "No se encontró la linea"));
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let option = args.get(1).unwrap().parse::<u32>();
    let filename = args.get(2).unwrap();
    let line = args.get(1).unwrap().parse::<u32>();
    let res = match option.unwrap() {
        ALL => read_all_file(filename),
        LINE => read_line_file(filename, line.unwrap()),
        _ => Result::Err(Error::new(ErrorKind::Other, "Opción desconocida")),
    };
    match res {
        Ok(c) => println!("El contenido leido es : {}", c),
        Err(e) => println!("{}", e),
    }
}
