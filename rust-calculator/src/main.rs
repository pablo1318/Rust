use std::io::{stdin, stdout, Write};

fn mostrar_opciones() {
    println!("0.SALIR");
    println!("1.SUMAR");
    println!("2.RESTAR");
    println!("3.MULTIPLICAR");
    println!("4.DIVIDIR");
    print!("Elija una opción de entre las posibles: ");
    let err = stdout().flush();
    assert_eq!(err.is_err(), false);
}
fn leer_operandos() -> (i32, i32) {
    let a: i32;
    let b: i32;
    let mut string_a = String::new();
    let mut string_b = String::new();
    print!("Escriba el primer operando: ");
    let err = stdout().flush();
    assert_eq!(err.is_err(), false);
    let option_read_error = stdin().read_line(&mut string_a);
    assert_eq!(option_read_error.is_err(), false);

    let mut r_a = string_a.trim_end().parse::<i32>();
    while r_a.is_err() {
        println!("Numero invalido, pruebe otra vez");
        let mut string_a = String::new();
        let option_read_error = stdin().read_line(&mut string_a);
        assert_eq!(option_read_error.is_err(), false);
        r_a = string_a.trim_end().parse::<i32>();
    }
    a = r_a.unwrap();
    print!("Escriba el segundo operando: ");
    let err = stdout().flush();
    assert_eq!(err.is_err(), false);
    let option_read_error = stdin().read_line(&mut string_b);
    assert_eq!(option_read_error.is_err(), false);

    let mut r_b = string_b.trim_end().parse::<i32>();
    while r_b.is_err() {
        println!("Numero invalido, pruebe otra vez");
        let mut string_b = String::new();
        let option_read_error = stdin().read_line(&mut string_b);
        assert_eq!(option_read_error.is_err(), false);
        r_b = string_b.trim_end().parse::<i32>();
    }
    b = r_b.unwrap();

    return (a, b);
}

#[derive(PartialEq)]
enum Resultado {
    Div0,
    Res(i32),
    Exit,
}

fn calcular(option: &i32) -> Resultado {
    let a: i32;
    let b: i32;

    if *option == 0 {
        return Resultado::Exit;
    }

    (a, b) = leer_operandos();

    println!("Los operandos leidos han sido: {} , {}", a, b);

    let resul: Resultado;
    match option {
        1 => {
            resul = Resultado::Res(a + b);
            return resul;
        }
        2 => {
            resul = Resultado::Res(a - b);
            return resul;
        }
        3 => {
            resul = Resultado::Res(a * b);
            return resul;
        }
        4 => {
            if b == 0 {
                return Resultado::Div0;
            }
            resul = Resultado::Res(a / b);
            return resul;
        }
        _ => {
            return Resultado::Div0; //aqui nunca se llega
        }
    }
}

fn main() {
    let mut numeric_option: i32;
    let mut r: Result<i32, std::num::ParseIntError>;
    loop {
        mostrar_opciones();
        let mut option = String::new();
        let option_read_error = stdin().read_line(&mut option);
        assert_eq!(option_read_error.is_err(), false);
        r = option.trim_end().parse::<i32>();
        if r.is_err() {
            println!("Error al parsear");
            continue;
        } else {
            numeric_option = r.unwrap();
        }
        if numeric_option < 0 || numeric_option > 4 {
            println!("POR FAVOR, ELIJA UNA OPCIÓN VÁLIDA");
            continue;
        }
        let r = calcular(&numeric_option);
        match r {
            Resultado::Res(v) => println!("El resultado ha sido {}", v),
            Resultado::Div0 => println!("No se puede dividir entre 0 maquina"),
            Resultado::Exit => {
                println!("SALIMOS!!!!!");
                break;
            }
        }
    }
}
