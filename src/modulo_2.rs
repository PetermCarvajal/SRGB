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

// ! Regla 3: Shadowing

pub fn ejercicio13(){

/*
TODO 13. Usa shadowing para sumar 1 a una variable inmutable que indica el nivel de un usuario, sin usar la palabra mut.
*/

    let lvlusuario:u8=5;
    let lvlusuario:u8=6;

    println!("{}",lvlusuario);

}

pub fn ejercicio14(){

/*
TODO 14. Usa shadowing para cambiar drásticamente el tipo de dato de una variable (inicia como texto "100" y hazle shadowing para que sea el número 100).
*/

    let shadowing:String=String::from("100");
    let shadowing:u8=100;

    println!("{}",shadowing);

}

pub fn ejercicio15() {

/*
TODO 15. Usa shadowing para extraer la longitud de un texto usando el método .len(), conservando exactamente el mismo nombre de la variable original.
*/

    let palabralarga:String = String::from("Lopadotemachoselachogaleokranioleipsanodrimhypotrimmatosilphioparaomelitokatakechymenokichlepikossyphophattoperisteralektryonoptekephalliokigklopeleiolagoiosiraiobaphetragano");

    let palabralarga:usize=palabralarga.len();

    println!("{}",palabralarga);

}

pub fn ejercicio16(){

/*
TODO 16. Declara una variable mut, modifícala matemáticamente, y luego usa shadowing (un nuevo let) para "congelarla" y volverla inmutable.
*/
    let mut numero:u8=5;
    numero*=2;

    let numero:u8=numero;

    println!("{}",numero);

}

pub fn ejercicio17(){

/*
TODO 17. Aplica shadowing múltiple en cascada (tres veces seguidas sobre la misma variable) para multiplicar, restar y dividir un número.
*/
    let mut numsrd:u8=15;
    let mut numsrd:u8 =numsrd*7;
    let mut numsrd:u8=numsrd-3;
    let mut numsrd:u8=numsrd/11;

    println!("{}",numsrd);

}

pub fn ejercicio18() {
    /*
    TODO 18. Demuestra el shadowing con bloques internos: crea una variable externa, hazle shadowing dentro de unas llaves { }, e imprímela dentro y fuera para ver la diferencia.
    */
    let mut global: u8 = 200;

    {
        let mut global: u8 = global + 10;
        println!("{}", global);
    }

    println!("{}",global);

}

// ! Regla 4: Enteros con signo

pub fn ejercicio19(){

/*
TODO 19. Declara un entero muy pequeño (i8) con un valor de temperatura bajo cero y muéstralo.
 */

    let entero:i8=-10;
    println!("{}",entero);

}

pub fn ejercicio20(){

/*
TODO 20. Declara un entero estándar (i32) para representar el saldo negativo de una cuenta bancaria.
*/

    let entero:i32=-100000;
}

pub fn ejercicio21(){

/*
TODO 	21. Declara un entero grande (i64) para representar el déficit económico de un país.
*/

    let deficit:i64=-25415412536;
    println!("{}",deficit);

}

pub fn ejercicio22(){

/*
TODO 22. Declara el entero más masivo disponible (i128) con un valor negativo de 30 dígitos.
*/

    let enteromas:i128=-123456789123456789123456789123;

}

pub fn ejercicio23(){

/*
TODO 23. Declara una variable de altitud negativa usando el sufijo de tipo directamente en el número (ej. -300i16) sin usar los dos puntos en el nombre.
*/

    let altitud=-30016;
    println!("{}",altitud);

}

pub fn ejercicio24(){

/*
TODO 24. Imprime el límite matemático mínimo (el número negativo más bajo) del tipo i32 usando la propiedad i32::MIN.
*/

    let mini:i32=i32::MIN;

    println!("{}",mini);

}