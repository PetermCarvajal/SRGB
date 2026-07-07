// info Dificultad Normal
// info Regla 1 (Inmutabilidad) + Regla 12 (println!)

pub fn ejercicio79(){

    /*
    TODO 79. Declara una variable inmutable con tu año de nacimiento y usa un println! para mostrar el mensaje "Nací en el año: [variable]".
	*/

    let año_nacimiento:u16=1999;

    println!("Nací en el año: {}",año_nacimiento);

}

pub fn ejercicio80(){

    /*
    TODO 80. Declara dos variables inmutables matemáticas (ej. precio y cantidad). En un solo println!, muestra el texto "Total a pagar: " seguido del resultado de multiplicarlas.
	*/

    let precio:u16=4700;
    let cantidad:u16=10;

    println!("Total a pagar: {}",precio*cantidad);

}

pub fn ejercicio81(){

    /*
    TODO 81. Crea una variable inmutable de tipo texto (&str) con el nombre de una ciudad. Imprime un println! que diga "Destino configurado: [ciudad]".
	*/

    let ciudad:&str="Bogota";

    println!("Destino configurado: {}",ciudad);

}

pub fn ejercicio82(){

    /*
    TODO 82. Declara tres variables inmutables numéricas para representar medidas (largo, ancho, profundidad). Imprime su volumen total calculándolo directamente dentro del println!.
    */

    let largo:u8=100;
    let ancho:u8=50;
    let profundidad:u8=25;

    println!("Volumen Total: {}",largo*ancho*profundidad);

}

pub fn ejercicio83(){

    /*
    TODO 83. Usa una variable inmutable para almacenar la versión de un software (ej. "v2.0"). Imprime un encabezado de bienvenida usando println! que incluya esa versión.
    */

    let version:&str="v2.0";

    println!("Version: {}",version);

}

pub fn ejercicio84(){

    /*
    TODO 84. Declara una variable inmutable usando una expresión matemática directamente en su asignación (ej. let dias = 365 * 2;). Imprime el resultado en una oración completa con println!.
    */

    let edad:u16=2026-1999;

    println!("Edad: {}",edad);

}

pub fn ejercicio85(){


}

pub fn ejercicio86(){

}

pub fn ejercicio87(){

}

pub fn ejercicio88(){

}

pub fn ejercicio89(){

}

pub fn ejercicio90(){

}

pub fn ejercicio91(){

}