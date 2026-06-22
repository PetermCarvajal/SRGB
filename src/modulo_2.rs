//! Regla 1: let Inmutabilidad

pub fn ejercicio1() {

/*
TODO 1.	Declara una variable inmutable llamada sistema de tipo texto implícito (str) y asígnale "Linux".
*/

    let sistema:&str="Linux";

    println!("{}",sistema);

}

pub fn ejercicio2(){

/*
TODO 2.	Declara dos variables inmutables matemáticas (base y altura) y muestra el resultado de su multiplicación directamente en un print.
*/

    let base:u8=4;
    let altura:u8=6;

    println!("{}x{}={}",base,altura,base*altura)

}

pub fn ejercicio3(){

/*
TODO 3.	Declara una variable decimal donde el compilador deduzca el tipo automáticamente e imprímela.
*/

    let automatico=3.1415;

    println!("{}",automatico);

}

pub fn ejercicio4(){

/*
TODO 4.	Almacena el resultado de la función String::from("Libro Rust") en un let inmutable y muéstralo.
*/

    let resultado=String::from("Libro Rust");

    println!("IDK {}",resultado);

}

pub fn ejercicio5(){

/*
TODO 5.	Declara una constante lógica inmutable explícita (bool) para representar si un servidor está activo.
*/

    let estado:bool=true;

}

pub fn ejercicio6(){

/*
TODO 6.	Declara un carácter Unicode inmutable que represente la clasificación de un videojuego (ej. 'M') e imprímelo.
*/

    let caracter:char='M';

    println!("{}", caracter);

}

// ! Regla 2: mut Mutabilidad


pub fn ejercicio7(){

/*
TODO 7.Crea una variable mutable numérica para la vida de un jugador, imprímela, réstale 50 puntos y vuelve a imprimirla.
*/
    let mut edad:u8=100;

    println!("{}",edad);

    edad=edad-50;

    println!("{}",edad);

}

pub fn ejercicio8(){

/*
TODO 8. Usa un operador de asignación compuesta (+=) con una variable mut para sumar 10 a un puntaje inicial.
*/

    let mut puntaje:i8=0;

    puntaje+=10;

    println!("{}",puntaje);

}

pub fn ejercicio9(){

/*
TODO 9. Cambia una variable booleana de estado de false a true simulando el encendido de un motor.
*/

    let mut estado:bool=false;
    estado=true;

    println!("{}",estado);

}

pub fn ejercicio10(){

/*
TODO 10. Muta una coordenada en formato flotante (f32) sumándole 2.5 a su valor original.
*/

    let mut ejex:f32=10.5;
    ejex+=2.5;

    println!("{}",ejex);

}

pub fn ejercicio11(){

/*
TODO 11. Crea un texto mutable usando String::from("Hola") y agrégale más texto usando el método .push_str(" Mundo").
*/

    let mut texto:String=String::from("Hola");
    (&mut texto).push_str(" Mundo");

    println!("{}",texto);

}

pub fn ejercicio12(){

/*
TODO 12. Declara una variable mut de tipo i16, asígnale el valor 500, y luego cámbialo a -500 para demostrar la conservación del tipo.
*/

    let mut desconocido:i16=500;
    desconocido=-500;

    println!("{}",desconocido);

}

