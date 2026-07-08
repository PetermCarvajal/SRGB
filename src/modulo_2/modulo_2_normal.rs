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

// info Regla 2 (Mutabilidad) + Regla 4 (Enteros sin Signo)

pub fn ejercicio85(){

    /*
    TODO 85. Declara una variable mutable de tipo i8 simulando la temperatura de un congelador (ej. -10). Réstale 5 grados y vuelve a guardarla.
    */

    let mut temperatura:i8=-10;
    temperatura=temperatura-5;

}

pub fn ejercicio86(){

    /*
    TODO 86. Crea un balance bancario mutable usando i32 con un saldo negativo inicial de -500. Súmale 1000 para sacarlo de números rojos.
    */

    let mut balance:i32=500;
    balance=balance+1000;

}

pub fn ejercicio87(){

    /*
    TODO 87. Declara la altitud de un submarino como mut i16 inicializada en 0. Haz que descienda restándole 150 metros en la siguiente línea.
    */

    let mut altitud:i16=0;
    altitud=altitud-150;

}

pub fn ejercicio88(){

    /*
    TODO 88. Usa un mut i64 para la deuda de una corporación multinacional (valor negativo). Simula un pago dividiendo la deuda a la mitad (/= 2).
    */

    let mut google:i64=-1500000000;
    google/=2;

}

pub fn ejercicio89(){

    /*
    TODO 89. Declara un contador de penalizaciones usando mut i8 inicializado en 0. Resta 1 por cada penalización en tres líneas consecutivas (terminando en -3).
    */

    let mut contador_penalizaciones:i8=0;
    contador_penalizaciones-=1;
    contador_penalizaciones-=1;
    contador_penalizaciones-=1;

}

pub fn ejercicio90(){

    /*
    TODO 90. Declara una variable mutable límite mut i32 y asígnale el valor de i32::MIN. Intenta sumarle 1 en la siguiente línea.
    */

    let mut mutable:i32=i32::MIN;
    mutable=mutable+1;

}

pub fn ejercicio91(){

}