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
}
