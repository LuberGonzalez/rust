// debug

#[allow(dead_code)]

// use rand::prelude::*;
// use std::cmp::Ordering;
// use std::io;

// funciones es rust sin retorno
fn second_funtion(x: i32) {
    println!("El numero es {x}");
}

fn suma(a: i32, b: i32) -> i32 {
    // retorno explicito no requiere ;
    a + b
    // retorno implicito require ; y el return
    //return a + b;
}

// fn game() {
//     // Juego de adivinanza del numero uso de librerias y como funciona rust con datos I/O
//     let mut num: String = String::new();
//     let secret_number = rand::rng().random_range(1..100);

//     println!("Adivina el nro:");
//     //println!("Numero secreto es {:?}", secret_number);

//     loop {
//         println!("Introduce un numero del 1 al 100 para adivinarlo");

//         io::stdin()
//             .read_line(&mut num)
//             .expect("No se pudo leer la linea");

//         // sin manejo de error
//         //let num: i32 = num.trim().parse().expect("Introduce un numero");

//         // con manejo de error si el numero no se puede parsear
//         let num: i32 = match num.trim().parse() {
//             Ok(num) => num,
//             Err(e) => {
//                 print!("El error es {e}");
//                 println!("El numero es {num}");
//                 //println!("Introduce un numero valido");
//                 continue;
//             }
//         };
//         println!("Tu prediccion es {num}",);

//         match num.cmp(&secret_number) {
//             Ordering::Equal => println!("Tu prediccion es Igual"),
//             Ordering::Greater => println!("Tu prediccion  es Gigante"),
//             Ordering::Less => {
//                 println!("Tu prediccion  es Pequeña");
//                 break;
//             }
//         }
//     }
// }

// tuplas con devolcion de dos valores

// fn cal_ln(text: String) -> (String, usize) {
//     let long = text.len();

//     (text, long)
// }

fn main() {
    println!("Clase de VEC Curso Pildoras Informaticas");

    //let num = vec![1, 2, 3];

    let mut vec_mut = Vec::new();

    vec_mut.push(10);
    vec_mut.push(20);
    vec_mut.push(30);

    let last = vec_mut.pop();

    // acceder a un elemento del vector
    print!("{:?}", vec_mut.get(4));

    print!("{:?}", vec_mut);
    print!("{:?}", last);

    // iterar vectores con for

    /* Al hacer un ciclo la propiedad
       se transfiera a un iterador en tiempo de ejecucion
       hay que implementar un borrowing (prestamo)
       con una ref
    */

    for n in &mut vec_mut {
        *n += 1;
    }

    // para aqui poder hacer la impresion
    print!("{:?}", vec_mut);

    let a: u8 = 100;
    let b: u8 = 200;

    /* Manejo de limite de memorias si de da el caso en donde hay errores no definidos */
    println!("a = {a}, b = {b}");

    // wrapping_add
    let wrapped = a.wrapping_add(b);
    println!("wrapping_add  => {wrapped}");

    // checked_add
    match a.checked_add(b) {
        Some(result) => println!("checked_add {result}"),
        None => println!("checked_add => Overflow None"),
    }

    // overflowing_add
    let (overflow_result, did_overflow) = a.overflowing_add(b);
    println!("overflowing_add => result: {overflow_result} did_overflow {did_overflow} ");

    // saturating_add maximo de capacidad
    let saturated_sum = a.saturating_add(b);
    println!("saturating_add {saturated_sum}");

    // tuplas
    let tup = (10, "hello", true);
    println!("{:?}", tup);

    let tup_second: (i32, f64, char) = (5234, 3.2, 'A');
    println!("{:?}", tup_second);

    // acceder individualmente por indice
    let first = tup_second.1;

    // let (second) = tup_second;

    println!("tup_second, {first}");

    // println!("second", { second });
    second_funtion(999);

    // retorno en funciones
    let result: i32 = suma(99, 10077);
    println!("Resultado de la suma: {result}");

    // expresiones de control
    let num = 10;
    let num_less = 5;

    if num > 5 {
        println!("El numero es mayor a {num_less}");
    } else {
        println!("El numero es menor a {num_less}");
    }

    // iteradores y loops
    let mut count = 0;

    loop {
        count += 1;
        println!("Contador es {count}");
        if count == 5 {
            break;
        }
    }

    let element = [1, 2, 3, 4, 5];

    for e in element {
        println!("Elemento es {e}");
    }

    println!("Ownership");

    let lit = "Hola";

    let type_string = String::from(lit);

    println!("EL valor es: {type_string}");

    // sin borrowing

    // let msg = String::from("Hello world");

    // let (new_msg, long) = cal_ln(msg);

    // println!("La longitud es {long} de la palabra {new_msg}");

    // funciones en rust

    // NLL Non Lexical Lifetimes

    let mut lexical = String::from("Hola");

    let ref_1 = &lexical;
    let ref_2 = &lexical;

    println!("{ref_1} {ref_2}");

    let ref_3 = &mut lexical;

    println!("{ref_3}");

    // slices Tipo especial se referencia con una [T]
    // si el principio es 0 se puede omitir y se coloca el segundo

    let frase = String::from("Bienvenido a Rust");

    let bienvenida = &frase[0..10];

    let lenguaje = &frase[13..17];

    println!("Primer slice: {}", bienvenida);

    println!("segundo slice: {}", lenguaje);

    // structs
    #[derive(Debug)]
    struct Samurai {
        nombre: String,
        clan: String,
        salud: u32,
        honor: i32,
    };

    let samurai_1 = Samurai {
        nombre: String::from("Musashi"),
        clan: String::from("Miyamoto"),
        salud: 100,
        honor: 50,
    };

    // Para hacer copias de structs se puede usar el operador .. para copiar los campos que no se especifican

    let samurai_2 = Samurai {
        nombre: String::from("Kenshin"),
        clan: String::from("Himura"),
        ..samurai_1
    };

    // println!("Samurai 1: {:?}", samurai_1);
    // println!("Samurai 2: {:?}", samurai_2);

    #[derive(Debug)]
    struct Coordenada(f64, f64);

    let Caracas = Coordenada(10.4806, 66.9036);

    println!(
        "Coordenada de Caracas: ({}, {}) mi ciudad natal.",
        Caracas.0, Caracas.1
    );

    #[derive(Debug)]
    struct Rectangulo {
        ancho: u32,
        alto: u32,
    }

    let escala = 3;

    let rect = Rectangulo {
        ancho: dbg!(20 * escala),
        alto: 80,
    };

    dbg!(&rect);
}
