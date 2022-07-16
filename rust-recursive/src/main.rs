use std::io::{stdin, stdout, Write};
use std::time::Instant;

fn empty_buff() {
    let r = stdout().flush();
    assert_eq!(r.is_err(), false);
}

fn calcular_fibonacci(n: u64) -> u64 {
    //println!("Se llama con la n: {}", n);
    match n {
        0 | 1 => n,
        _ => calcular_fibonacci(n - 1) + calcular_fibonacci(n - 2),
    }
}

fn calcular_fibonacci_iter(n: u64) -> u64 {
    let mut r = 1;
    let mut previous_number: u64 = 0;
    let mut previous_previous_number: u64;
    for _ in 1..n {
        previous_previous_number = previous_number;
        previous_number = r;
        r = previous_previous_number + previous_number;
    }
    return r;
}

fn main() {
    print!("Escriba un número para hacer la sucesión de Fibonacci: ");
    empty_buff();
    let mut buf = String::new();

    let n = stdin().read_line(&mut buf);
    if n.is_err() {
        eprintln!("ALGUN ERROR AL LEER");
        return;
    }
    let num = buf.trim_end().parse::<u64>();
    if num.is_err() {
        eprintln!("ALGUN ERROR AL PARSEAR, NUMERO INVALIDO");
        return;
    }
    let n1 = num.unwrap();
    let time = Instant::now();

    let res = calcular_fibonacci(n1);
    let dur = time.elapsed();
    println!(
        "El resultado del cálculo RECURSIVO ha sido : {}, calculado en {} microsegundos",
        res,
        dur.as_micros()
    );

    let time = Instant::now();
    let res = calcular_fibonacci_iter(n1);
    let dur = time.elapsed();
    println!(
        "El resultado del cálculo ITERATIVO ha sido : {}, calculado en {}",
        res,
        dur.as_micros()
    );
}
