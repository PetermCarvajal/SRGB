// ! Reglas 14: Tuplas

pub fn ejercicio_1() {
    /*
    TODO  1.	Declara una tupla inmutable llamada configuracion que contenga un entero i32 y un booleano bool. Imprímela usando el formateador de depuración {:?}.
    */

    let configuracion:(i32,bool) = (27,true);

    println!("Formateador de Depuración {:?}", configuracion);

}

pub fn ejercicio_2(){

    /*
    TODO 2.	Crea una tupla matemática que represente un vector 3D (f64, f64, f64) inicializada con los valores de origen (0.0). Imprímela.
    */

    let vector:(f64,f64,f64) = (0.0,0.0,0.0);

    println!("{:?}",vector);

}

pub fn ejercicio_3(){

    /*
    TODO 3.	Declara una tupla que contenga tu edad (u8) y la inicial de tu nombre (char). Imprímela.
    */

    let myself:(u8,char) = (27u8,'P');

    println!("{:?}",myself);

}

pub fn ejercicio_4(){

    /*
    TODO 4.	Demuestra que el compilador infiere tipos creando una tupla sin especificar los tipos explícitamente: let datos = ("Sistema", 2026); e imprímela.
    */

    let datos=("Sistema",2026);

    println!("{:?}",datos);

}

pub fn ejercicio_5(){

    /*
    TODO 5.	Declara una tupla anidada (una tupla dentro de otra tupla): let anidada = ( (1, 2), 3 ); e imprímela.
    */

    let tupla_anidada = ((1,2),3);

    println!("{:?}",tupla_anidada);

}

pub fn ejercicio_6(){

    /*
    TODO 6.	Crea una tupla que mezcle 4 tipos primitivos distintos: i16, f32, bool, y char. Imprímela visualizando todos sus datos.
    */

    let tipos_primitivos:(i16,f32,bool,char) = (16,6.48,false,'P');

    println!("{:?}",tipos_primitivos);

}

// ! Regla 15: Destructuración de Tuplas

pub fn ejercicio_7(){

    /*
   TODO 7. Crea una tupla (100, 200) y destrúcturala en dos variables separadas llamadas ancho y alto. Imprime la variable alto usando un formateador normal {}.
    */

    let tupla:(u8,u8) = (100,200);

    let (var1,var2)=tupla;

    println!("{:?}",tupla);
    println!("{:?}",var2);

}

pub fn ejercicio_8() {

    /*
    TODO 8. Declara una tupla con 3 caracteres Unicode simulando una contraseña (usa 3: 'A', 'B', 'C'). Destrúcturala en char1, char2, char3 e imprime el segundo.
    */

    let pass:(char,char,char)=('A','B','C');

    let (var1,var2,var3)=pass;

    println!("{:?}",var2);

}

pub fn ejercicio_9() {

    /*
    TODO 9. Destructura una tupla de tipo (f64, bool) que represente (velocidad, limite_excedido). Imprime un mensaje interpolando ambas variables extraídas.
   */

    let tupla_tipo:(f64,bool)=(200.0,true);

    let (var1,var2)=tupla_tipo;

    println!("Usted iva a {:?}, ¿Limite superado? {:?}",var1,var2);

}

pub fn ejercicio_10(){

    /*
    TODO 10. Usa destructuración para extraer solo el primer valor de una tupla de 3 elementos, ignorando el resto (usa variables _ o nombres genéricos para los que no uses, pero extráelos todos en la sintaxis de igualación).
    */

    let persona:(&str,u8,u32)=("Peter",27,3003003001);

    let (nombre,edad,cel)=persona;

    println!("{:?}",persona);

    println!("Primero: {:?}",nombre)

}


pub fn ejercicio_11(){

    /*
    TODO 11. Declara una tupla con valores de servidor ("192.168.1.1", 8080) y destrúcturala en ip y puerto. Muestra el puerto.
     */

    let servidor :(&str,u16)=("192.168.1.1", 8080);

    let (ip,puerto)=servidor;

    println!("{:?}",puerto);

}

pub fn ejercicio_12(){

    /*
    TODO 12. Demuestra la destructuración con una operación matemática: extrae (a, b) de la tupla (10, 5) y en la siguiente línea imprime la suma de a + b.
     */

    let suma:(u8,u8)=(10,5);

    let (a,b)=suma;

    println!("{:?}",a+b);

}

pub fn ejercicio_13(){

    /*
    TODO 13. Declara una tupla (50.0, 100.0) y accede a su primer elemento usando la notación de punto .0 para imprimirlo.
     */

    let tupla :(f64,f64)=(50.0,100.0);

    println!("Primer elemento: {:?}",tupla.0);

}

pub fn ejercicio_14() {

    /*
   TODO 14. Crea una tupla inmutable de tres booleanos (true, false, true). Imprime el último elemento usando el índice .2.
     */

    let booleanos:(bool,bool,bool)=(true,false,true);


    println!("{:?}",booleanos.2);

}

pub fn ejercicio_15(){

    /*
    TODO 15. Declara una tupla mutable mut jugador = ("Knight", 1). Muta el segundo elemento (el nivel) para sumarle 1 usando .1. Imprime la tupla completa.

     */

    let mut jugador:(&str,u8)=("Knight",1);

    print!("{:?}",jugador.1+1);

}

pub fn ejercicio_16(){

    /*
    TODO 16. Declara una tupla de coordenadas (15, 30) y suma sus dos valores accediendo a ellos por sus índices en un println!.
     */

    let coordenadas:(u8,u8)=(15,30);

    println!("{:?}",coordenadas.1+coordenadas.0);

}

pub fn ejercicio_17(){

    /*
    TODO 17. Crea una tupla anidada let matriz = ((1, 2), (3, 4));. Accede e imprime el número 4 encadenando los índices (ej. .1.1).
     */

    let tupla_anidada:((u8,u8),(u8,u8))=((1,2),(3,4));

    println!("{:?}",tupla_anidada);

    println!("Imprimiendo el ultimo: {:?}",tupla_anidada.1.1);

}

pub fn ejercicio_18(){

    /*
    TODO 18. Declara una tupla mutable con un carácter de estado ('E', 404). Cambia la 'E' a 'O' (de OK) usando .0 e imprímela.
     */

    let mut tupla_mutable:(char,u16)=('E',404);

    println!("Tupla Mutable {:?}",tupla_mutable);

    tupla_mutable=('O',404);

    println!("{:?}",tupla_mutable.0);

}

pub fn ejercicio_19(){

    /*
    TODO 19. Declara un array explícito llamado dias que contenga 7 enteros de tipo u8. Imprímelo usando {:?}.
20. Declara un array de tipo [f32; 3] simulando la posición de un modelo 3D [1.5, 0.0, -1.5]. Imprímelo.
21. Crea un array que dependa de la inferencia del compilador (sin poner los tipos): let numeros = [10, 20, 30]; e imprímelo.
22. Declara un array de 5 booleanos inicializados aleatoriamente en true o false. Imprímelo en consola.
23. Declara un array estricto de caracteres [char; 4] con las letras 'A', 'B', 'C', 'D'. Imprímelo.
24. Declara un array inmutable de 2 elementos tipo texto &str ("Usuario", "Admin") e imprímelo
     */

    let nombres:[&str;2]=["Peter","Sua"];

}