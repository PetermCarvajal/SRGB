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
8. Declara una tupla con 3 caracteres Unicode simulando una contraseña ('R', 'u', 's', 't') (espera, son 4, usa 3: 'A', 'B', 'C'). Destrúcturala en char1, char2, char3 e imprime el segundo.
9. Destructura una tupla de tipo (f64, bool) que represente (velocidad, limite_excedido). Imprime un mensaje interpolando ambas variables extraídas.
10. Usa destructuración para extraer solo el primer valor de una tupla de 3 elementos, ignorando el resto (usa variables _ o nombres genéricos para los que no uses, pero extráelos todos en la sintaxis de igualación).
11. Declara una tupla con valores de servidor ("192.168.1.1", 8080) y destrúcturala en ip y puerto. Muestra el puerto.
12. Demuestra la destructuración con una operación matemática: extrae (a, b) de la tupla (10, 5) y en la siguiente línea imprime la suma de a + b.
 */

    let tupla:(u8,u8) = (100,200);


}