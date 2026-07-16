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

// info R3 (Shadowing) + R7 (Flotantes Simples f32)

pub fn ejercicio91(){

    /*
     TODO 91. Declara una variable inmutable f32 para un precio base. Usa shadowing (let nuevamente) para aplicarle un impuesto multiplicándola por 1.15.
	*/

    let precio:f32=200.000;

    let precio:f32=precio*1.15;

}

pub fn ejercicio92(){

    /*
    TODO 92. Usa shadowing para transformar una variable que inicia como el número entero 10 a una nueva variable con el mismo nombre pero de tipo flotante (10.0f32).
	 */

    let edad:u8=23;

    let edad:f32=23.0;

}


pub fn ejercicio93(){

    /*
    TODO 93. Calcula la velocidad constante ($v = d / t$). Declara $d$ y $t$ como enteros. Usa shadowing sobre una variable velocidad para forzar que el resultado final sea un f32.
     */

    let d:i8=30;

    let t:i8=5;

    let velocidad:f32=d as f32/t as f32;

}

pub fn ejercicio94() {

/*
TODO 94. Declara una temperatura en grados Celsius como f32. Usa shadowing sobre la misma variable matemática para convertir su valor a Fahrenheit ($C \times 1.8 + 32$).
 */

     let temperatura:f32=0.0;

    let temperatura:f32=(temperatura*1.8)+32 as f32;

}

pub fn ejercicio95() {

    /*
    TODO 95. Crea un bloque interno { }. Fuera del bloque declara una gravedad de 9.8f32. Dentro del bloque, usa shadowing para cambiarla temporalmente a gravedad marciana 3.7f32.
     */

    let gravedad:f32=9.8;
     println!("gravedad: {}",gravedad);
    {
        let gravedad:f32=3.7;
        println!("gravedad: {}",gravedad);
    }

}

pub fn ejercicio96() {

    /*
    TODO 96. Declara el radio de un círculo como texto ("5.5"). Usa shadowing simulado para crear la variable matemática radio como f32 (asignándole directamente el número 5.5).
     */

    let radio:&str="5.5";

    let radio:f32=5.5;

}

// info Regla R5 (Enteros sin Signo u8 a u128) + Regla R9 (Booleanos bool)

pub fn ejercicio97() {

    /*
    TODO 97. Declara una edad humana u8. En otra variable, crea un bool que evalúe si la edad es mayor o igual a 18.
     */

    let edad:u8=13;

    let verificar:bool=edad>=18;

}

pub fn ejercicio98() {

    /*
    TODO 98. Declara los puntos de vida de un enemigo como u16. Crea una variable booleana que sea true si los puntos de vida son exactamente 0 (usando ==).
     */

    let salud_enemigo:u16=300;

    let muerto:bool=salud_enemigo==0;

}

pub fn ejercicio99() {

    /*
    TODO 99. Declara el límite de velocidad de una vía como u8 (ej. 80) y la velocidad de un auto como u8 (ej. 95). Genera un bool que indique si hay infracción.
     */

    let limite:u8=80;

    let velocidad_auto:i8=95;

    let multa:bool=limite>velocidad_auto as u8;

}

pub fn ejercicio100(){

    /*
    TODO 100. Usa un u32 para el número de intentos de login fallidos. Crea un booleano cuenta_bloqueada que sea verdadero si los intentos son mayores a 3.
     */

    let intentos:u32=2;

    let bloquea_cuenta:bool=intentos >3;

}

pub fn ejercicio101() {
    /*
    TODO 101. Declara un u64 para representar el espacio libre en un disco duro. Genera un booleano llamado alerta_espacio que evalúe si el espacio es menor a 1000.
     */

    let espacio_libre: u16 = 100;

    let advertencia:bool=espacio_libre<1000;

}

pub fn ejercicio102() {

    /*
    TODO 102. Declara el color RGB rojo puro usando un u8 (255). Crea un booleano que verifique si el valor es distinto (!=) de 0.
     */

    let RGB:u8=255;

    let distinto:bool=RGB!=0;

}

// info Regla 6 (Arquitectura usize / isize) + Regla 11 (print! sin salto)

pub fn ejercicio103(){

    /*
    TODO 103. Declara una variable usize con el número total de archivos a procesar. Usa print! para escribir "Procesando archivo 1 de [variable]...".
	*/

    let numero_total:usize=12165321354;

    print!("Procesando archivo 1 de {numero_total}");

}

pub fn ejercicio104(){

    /*
    TODO 104. Declara un índice de retroceso de memoria de tipo isize (-4). Imprime su valor junto con el texto "Desplazamiento aplicado: " en la misma línea usando print!.
     */

    let retroceder_memoria:isize=-4;

    print!("Desplazamiento aplicado: ");
    print!("{retroceder_memoria}")

}

pub fn ejercicio105() {

    /*
    TODO 105. Obtén la capacidad máxima de punteros usando usize::MAX e imprímela usando dos macros print! separados que formen una sola frase.
     */

    let capacidad_max:usize=usize::MAX;

    print!("La capacidad máxima de punteros con usize es de :");
    print!("{capacidad_max}");

}

pub fn ejercicio106(){

    /*
    TODO 106. Declara una posición de array en usize (ej. 5). Usa print! para simular la búsqueda: "Buscando en índice [posición]".
     */

    let pos_actual:usize=5;

    print!("Buscando índice {}",pos_actual);

}

pub fn ejercicio107(){

    /*
    TODO 107. Combina la declaración explícita 500usize con un print! que diga "Bloque de memoria asignado: ".
     */

    let explicito=500usize;

    print!("Bloque de memoria asignado {explicito}");

}

pub fn ejercicio108(){

    /*
    TODO 108. Declara un delta de movimiento isize positivo y luego negativo en dos variables distintas. Imprímelos consecutivos sin salto de línea.
     */

    let delta_mov:isize=123;

    let alpha_mov:isize=-124;

    print!("{},{}",delta_mov,alpha_mov);

}

// info Regla R8 (Flotantes Dobles f64) + Regla R13 (eprintln! Errores)

pub fn ejercicio109(){

    /*
    TODO 109. Declara un f64 extremadamente preciso representando una desviación en la órbita. Usa eprintln! para mostrar "Peligro: Desviación detectada de [variable]".
     */

    let desviacion_orbita:f64=3.141516;

    eprintln!("Desviación detectada de {desviacion_orbita}");

}

pub fn ejercicio110(){

    /*
    TODO 110. Ocurrió un error financiero por precisión. Declara una diferencia contable usando un f64 negativo e imprímela usando eprintln! limitando los decimales a {:.4}.
     */

    let diferencia_contable:f64=9.050000;

    eprintln!("{:.4}",diferencia_contable);

}

pub fn ejercicio111(){

    /*
    TODO 111. Declara la constante matemática Pi en f64. Usa eprintln! para mostrar un error simulado: "Error de cálculo: Pi se asumió como [variable]".
     */

    const PI:f64=4.141516;

    eprintln!("Error de cálculo: Pi se asumió como {PI}");

}

pub fn ejercicio112(){

    /*
    TODO 112. Un sensor de radiación falló. Asigna un nivel de lectura anormal usando notación científica f64 (ej. 1.5e6) y envíalo al log de errores con eprintln!.
     */

    let lectura:f64=1.5e6;

    eprintln!("{lectura}");

}

pub fn ejercicio113(){

    /*
    TODO 113. Genera un error crítico usando eprintln! e interpola directamente una operación matemática de dos variables f64 dentro del macro.
     */

    let precio_accion:f64=213.2;

    let tiempo_m:f64=60.0*45.0;

    eprintln!("Perdidas monetarias ascienden {}",precio_accion*tiempo_m);

}

pub fn ejercicio114(){

    /*
    TODO 114. Declara el límite de tiempo de espera (timeout) de un servidor en f64 (ej. 30.0). Muestra un error indicando "Timeout agotado tras [variable] segundos".
     */

    let timeout:f64=30.0;

    eprintln!("Tiempo agotado tras {timeout}");

}

// info Regla 10 (char) + Regla 2 (mut  Mutabilidad)

pub fn ejercicio115(){

    /*
    TODO 115. Declara una calificación mutable de tipo char inicializada en 'B'. Cámbiala a 'A' simulando una corrección de nota.
     */

    let mut calificacion:char='B';

    calificacion = 'A';

}

pub fn ejercicio116(){

    /*
    TODO 116. Crea el estado de un semáforo usando una variable mutable char ('V' para verde). Mútalo a 'A' (amarillo) y luego a 'R' (rojo).
     */

    let mut estado:char='V';

    estado='A';

    estado='R';

}

pub fn ejercicio117(){

    /*
    TODO 117. Inicializa una variable mut char con un emoji de cara triste 😢. Reasígnala para que contenga un emoji de cara feliz 😊.
     */

    let mut emoji:char='😢';

    emoji='😊';

}

pub fn ejercicio118(){

    /*
    TODO 118. Declara la dirección de un jugador como un mut char ('N' para Norte). Modifica su dirección a 'E' (Este).
     */

    let mut direccion:char='N';

    direccion='E';

}

pub fn ejercicio119(){

    /*
    TODO 119. Crea un carácter mutable que represente una letra minúscula 'a'. (Aunque Rust tiene métodos para mayúsculas, simplemente reasígnala manualmente a 'A').
     */

    let mut letra:char='a';

    letra='A';

}

pub fn ejercicio120(){

    /*
    TODO 120. Declara un símbolo matemático como mut char (ej. '+'). Mútalo para representar una resta ('-') en la siguiente línea.
     */

    let mut simbolo:char='+';

    simbolo='-';

}

