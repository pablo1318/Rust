use std::{env, fs};
fn read_all_file(path: String) {
    let content = fs::read_to_string(path);
    match content {
        Ok(a) => println!("{}", a),
        Err(_) => println!("Ha habido un error, fichero invalido"),
    }
}

fn read_line_file(path: String) {}
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Escribe solo un argumento(el fichero que quieres abrir)");
        return;
    }
    let filename = args.get(1).unwrap();
    read_all_file(filename.to_string());
}
