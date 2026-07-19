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

    let _estado:bool=true;

    println!("{}",_estado);

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

    let mut _estado:bool=false;

    println!("{}",_estado);

    _estado=true;

    println!("{}",_estado);

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
TODO 11. Crea un texto mutable usando String::from("Hola") y agrégale más texto usando el método .pushstr(" Mundo").
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
    println!("{desconocido}");

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

// ! Regla 5:  sin signo

pub fn ejercicio25(){
/*
TODO 25. Declara un byte puro (u8) con el valor máximo permitido (255) simulando el color azul.
*/

    let u8max:u8=255;

    println!("{}",u8max);

}

pub fn ejercicio26(){

/*
TODO 26. Declara una variable para un puerto de red usando u16 (ej. 443).
*/

    let puerto:u16=443;

    println!("{}",puerto);

}

pub fn ejercicio27(){

/*
TODO 27. Declara la edad de un edificio garantizando que nunca sea negativa usando u32.
*/

    let edad:u32=20;

    println!("{}",edad);

}

pub fn ejercicio28(){

/*
TODO 28. Declara un identificador único (ID) gigante de una base de datos usando u64.
*/

    let id:u64=1574856654656;

    println!("{}",id);
}

pub fn ejercicio29(){

/*
TODO 29. Declara el número de estrellas en una galaxia usando guiones bajos como separadores visuales para facilitar su lectura (ej. 100000000).
*/

    let estrellas:u64=100000000;

    println!("{}",estrellas);

}

pub fn ejercicio30(){

/*
TODO 30. Imprime el valor máximo que cabe en un u16 usando la propiedad u16::MAX.

*/

    let maxu16:u16=u16::MAX;

    println!("{}",maxu16);

}

// ! Regla 6: Enteros dependientes de Arquitectura

pub fn ejercicio31(){

/*
TODO 31. Declara un índice de posición de un menú (usize) inicializado en 0.
*/

    let menu:usize=0;

    println!("{}",menu);

}

pub fn ejercicio32(){

/*
TODO 32. Declara un desplazamiento de puntero en memoria que retroceda 8 posiciones usando isize.
*/

    let desplazamiento:isize=-8;

    println!("{}",desplazamiento);

}

pub fn ejercicio33(){

/*
TODO 33. Infiere el tipo usize usando el sufijo directamente en el número (ej. 500usize) para simular la capacidad de un inventario.
*/

    let infiere:usize=500usize;

    println!("{}",infiere);

}

pub fn ejercicio34(){

/*
TODO 34. Asigna el tamaño en bytes de una descarga usando usize y separadores visuales.
*/

    let descaraga:usize=1024000usize;

    println!("{}",descaraga);

}

pub fn ejercicio35(){

/*
TODO 35. Demuestra el uso de isize sumando un índice actual (positivo) con un salto relativo (negativo) y mostrando el resultado.
*/

    let indiceactual:isize=16;
    let saltorelativo:isize=-3;

    println!("{} + '('{}')' = {})",indiceactual,saltorelativo,indiceactual+saltorelativo);

}

pub fn ejercicio36(){

/*
TODO 36. Imprime la capacidad máxima de tu procesador para indexar memoria usando usize::MAX.
*/

    let capacidadmaxima:usize=usize::MAX;

    println!("{}",capacidadmaxima);

}

// ! Regla 7: Flotantes de Precisión Simple

pub fn ejercicio37(){

/*
TODO 37. Declara una variable de velocidad del viento con un decimal explícito usando f32.
*/

let velviento:f32=3.2;

println!("{}",velviento);

}

pub fn ejercicio38(){

/*
TODO 38. Declara un número entero, pero obliga al compilador a tratarlo como decimal añadiendo .0 al final, especificando tipo f32.
*/

    let intimpostor:f32=5.0;

    println!("{}",intimpostor);

}

pub fn ejercicio39(){

/*
TODO 39. Declara un f32 usando notación científica (ej. 1.5e-3) para simular la masa de un insecto.
*/

    let notacioncientifica:f32=1.5e-3;

    println!("{}",notacioncientifica);

}

pub fn ejercicio40(){

/*
TODO 40. Usa el sufijo de tipo para flotantes directamente en el número (ej. -12.5f32) para una coordenada X.
*/

    let flotanteexplicito:f32=-12.5f32;

    println!("{}",flotanteexplicito);

}

pub fn ejercicio41(){

/*
TODO 41. Realiza y muestra la multiplicación entre dos variables f32 (cantidad y peso).
*/

    let  cantidad:f32=2.1;

    let peso:f32=4.2;

    println!("{} x {} = {}",cantidad,peso,cantidad*peso);

}


pub fn ejercicio42(){

/*
TODO 42. Divide un f32 (ej. 10.0) entre 0.0 e imprime el resultado para observar cómo Rust maneja el infinito por hardware.
*/

    let division:f32=10.0;

    let entre:f32=0.0;

    println!("{} / {} = {} ",division,entre,division/entre);

}

// ! Regla R8: Flotantes de Precisión Doble

pub fn ejercicio43(){

/*
TODO 43. Declara un número decimal sin indicar el tipo para demostrar que Rust asigna f64 por defecto.
*/

    let nada=6.0;

    println!("{}",nada);

}

pub fn ejercicio44(){

/*
TODO 44. Declara una coordenada GPS de longitud con al menos 12 decimales de precisión usando f64.
*/

    let coordernadaGPS:f64=1.123456789321;

    println!("{}",coordernadaGPS);

}

pub fn ejercicio45(){

/*
TODO 45. Usa notación científica para calcular la distancia entre dos planetas (ej. 2.25e8).
*/

    let distanciaplanetaria:f64=2.5e8;

    let otradistancia:f64=13.9e18;

    println!("{}-{}={}",otradistancia,distanciaplanetaria,otradistancia-distanciaplanetaria);

}

pub fn ejercicio46(){

/*
TODO 46. Asigna explícitamente el tipo f64 a un número negativo cerrado (ej. -100.0).
*/

    let explicitamente:f64=-100.0;

    println!("{}",explicitamente);

}

pub fn ejercicio47(){

/*
TODO 47. Imprime un f64 (que tenga muchos decimales) limitando visualmente la salida a solo 2 decimales usando el formateador {:.2}.
*/

    let muchosdecimales:f64=1.131231231231231231231321231231231323123123123123;

    println!("Limitar la cantidad de Decimales Visibles :\n Original{}\nLimitado (:.# ejemplo :.2)\n{:.2}",muchosdecimales,muchosdecimales);

}

pub fn ejercicio48(){

/*
TODO 48. Calcula el área de un círculo usando un f64 para el radio y un f64 altamente preciso para el valor de Pi.
*/

    let radio:f64=4.1256;

    let pi:f64=3.141592653589793;

    let cuadrado:f64=radio*radio;

    println!("El area del Circulo es: {}",pi*cuadrado);

}

// ! Regla 9: Booleanos

pub fn ejercicio49(){

/*
TODO 49. Declara un booleano explícito que indique si un juego ha terminado (true).
*/

    let estadojuego:bool=true;

    print!("{}",estadojuego);

}

pub fn ejercicio50(){

/*
TODO 50. Declara un booleano explícito que indique si hay conexión a internet (false).
*/

    let estadointernet:bool=false;

    print!("{}",estadointernet);

}

pub fn ejercicio51(){

/*
TODO 51. Genera y guarda en una variable un booleano evaluando si una clave ingresada es igual (==) a la clave guardada.
*/

    let pinguardado:String=String::from("123456");

    let mut claveingresada:String=String::new();

    println!("Ingrese el Pin");

    let esvalido:bool=pinguardado==claveingresada;

    println!("¿La Contraseña es Correcta? = {}",esvalido);

}

pub fn ejercicio52(){

/*
TODO 52. Genera un booleano evaluando si la suma de 2 + 2 es diferente (!=) de 5.
*/

    let diferente:bool=(2+2)!=5;

    println!("{}",diferente);

}

pub fn ejercicio53() {

/*
TODO 53. Genera un booleano evaluando si la experiencia actual del jugador es mayor o igual (>=) a la experiencia requerida para subir de nivel.
*/

    let xprequerida: u8 = 254;

    let xpactual: u8 = 255;

    let cumple: bool = xpactual >= xprequerida;

    println!("{}", cumple);

}

pub fn ejercicio54(){

/*
TODO 54. Invierte el valor de un booleano de estado de alarma usando el operador lógico de negación (!).
*/

    let estadoalarma:bool=false;

    let estadoalarmainterrumptor:bool=!estadoalarma;

    println!("{}",estadoalarmainterrumptor);
    
}

// ! Regla 10: Char

pub fn ejercicio55(){

/*
TODO 55. Declara una vocal simple usando comillas simples.
*/

    let vocal:char='a';

    println!("{}",vocal);

}

pub fn ejercicio56(){

/*
TODO 56. Declara un número como carácter visual (entre comillas simples) para representar la etiqueta de un botón.
*/

    let numero:char='1';

    println!("{}",numero);

}

pub fn ejercicio57(){

/*
TODO 57. Declara un emoji (ej. 🚀) explícitamente como char.
*/

    let emoji:char='🚀';

    println!("{}",emoji);

}

pub fn ejercicio58(){

/*
TODO 58. Declara un espacio en blanco estricto como char.
*/

    let espacio:char=' ';

    println!("{}",espacio);

}

pub fn ejercicio59(){

/*
TODO 59. Declara una letra del alfabeto griego (ej. 'Ω') para demostrar el soporte Unicode de 4 bytes.
*/

    let omega:char='Ω';

    println!("{}",omega);

}

pub fn ejercicio60(){

/*
TODO 60. Imprime dos variables de tipo char distintas, una al lado de la otra, sin espacios en el formato de salida.
*/

    let char1:char='P';
    let char2:char='N';

    println!("{}{}",char1,char2);

}

// ! Regla 11: Print! sin salto de linea

pub fn ejercicio61(){

/*
TODO 61. Imprime la palabra "Iniciando" sin que la consola salte de línea.
*/

    print!("Iniciando");

}

pub fn ejercicio62(){
/*
TODO 62. Imprime tres palabras separadas usando tres sentencias print! consecutivas para que formen una sola frase.
 */
    print!("Una ");
    print!("Sola ");
    print!("Frase");

}

pub fn ejercicio63(){
/*
TODO 63. Usa print! para simular la etiqueta de un formulario ("Nombre de usuario: ") dejando el cursor al lado.
*/
    print!("Nombre de usuario:");

}

pub fn ejercicio64(){

/*
TODO 64. Imprime una cadena literal y una variable tipo str en una misma línea usando un solo print!.
 */

    let variable:&str="Keep";

    print!("{} calm and aprovecha el bug", variable);

}

pub fn ejercicio65(){

/*
TODO 65. Simula visualmente el inicio de una barra de carga imprimiendo [#### sin salto.
*/

    println!("Cargando");
    print!("[");
    print!("#");
    print!("#");
    print!("#");
    print!("#");
    print!("#");
    print!("#");
    print!("#");
    print!("#");
    print!("#");
    print!("#");
    print!("#");
    print!("#");
    print!("#");
    print!("#");
    print!("#");
    print!("#");
    print!("#");
    print!("]");

}

pub fn ejercicio66(){

/*
TODO 66. Ejecuta un print! que muestre un estado de carga, y en la siguiente línea de código otro print! que añada " OK".
*/

    println!("Cargando");
    print!("[");
    print!("#");
    print!("#");
    print!("#");
    print!("#");
    print!("#");
    print!("#");
    print!("#");
    print!("#");
    print!("#");
    print!("#");
    print!("#");
    print!("#");
    print!("#");
    print!("#");
    print!("#");
    print!("#");
    print!("#");
    print!("]");
    println!("OK");

}

// ! Regla 12: println! con salto de linea

pub fn ejercicio67() {

/*
TODO 67. Imprime el mensaje "Fin del programa" asegurando un salto de línea automático.
*/

    println!("Fin del programa");
}

pub fn ejercicio68(){

/*
TODO 68. Imprime dos variables numéricas en dos líneas diferentes de la consola usando dos sentencias println!.
*/

    let a:u8=10;
    let b:u8=3;

    println!("{}",a);
    println!("{}",b);

}

pub fn ejercicio69(){

/*
TODO 69. Interpola una variable de texto dentro de una oración completa en un println!.
*/

    let interpolar:&str="Dios si eres ateo";

    println!("Como puedes hablar de {}, en tus ojos lo veo mientras mi candela te consume.",interpolar);

}

pub fn ejercicio70(){

/*
TODO 70. Interpola tres variables (día, mes, año) en un solo println! formateado con barras separadoras (ej. 12/05/2026).
*/
    let dia:u8=7;
    let mes:u8=07;
    let año:u16=2026;

    println!("{}/{}/{}",dia,mes,año);

}

pub fn ejercicio71(){

/*
TODO 71. Realiza una operación matemática de suma directamente dentro de las llaves {} del macro println!.
*/

    println!("{} + {} = {} ",600000,700000,600000+700000);

}

pub fn ejercicio72(){

/*
TODO 72. Usa println!() completamente vacío para generar una línea de separación visual en la consola.
*/

    println!();

}

// ! Regla 13: impresión de errores

pub fn ejercicio73(){

/*
TODO 73. Imprime un mensaje estático de "Fallo en el sistema central" dirigido a la salida de errores.
*/

    eprintln!("Fallo en el sistema central");

}

pub fn ejercicio74(){

/*
TODO 74. Interpola el número del puerto (ej. 8080) en un mensaje de error usando eprintln!.
*/

    eprintln!("El puerto {} no responde",8080);

}

pub fn ejercicio75(){

/*
TODO 75. Combina una variable de texto (nombre del módulo) y un código de error numérico en un solo eprintln!.
*/
    let modulo:&str="X1125";
    eprintln!("El módulo {} informa sobre el código de error {}",modulo,1111);

}

pub fn ejercicio76(){

/*
TODO 76. Imprime dos errores críticos consecutivos para confirmar que, al igual que println!, también realizan salto de línea automático.
*/

    eprintln!("Código de error: delfin");
    eprintln!("Código de erorr: tortuga");

}

pub fn ejercicio77(){

/*
TODO 77. Interpola una variable char para indicar que el "Sector 'X' está comprometido" en la salida de errores.
*/

    let char:char='X';

    println!("Sector {} está comprometido",char);

}
pub fn ejercicio78(){

/*
TODO 78. Crea una variable bool en false y métela en un mensaje de eprintln! indicando que el "Estado de recuperación es: false".
*/

    let bin:bool=false;

    eprintln!("Estado de recuperación es: {}",bin);

}