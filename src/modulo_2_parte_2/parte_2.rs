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

// ! Regla 16:Acceso a Tuplas por Índice

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

// ! Regla 17: Arrays

pub fn ejercicio_19(){

    /*
    TODO 19. Declara un array explícito llamado dias que contenga 7 enteros de tipo u8. Imprímelo usando {:?}.
     */

    let dias:[u8;7]=[1,2,3,4,5,6,7];

    println!("{:?}",dias);

}

pub fn ejercicio_20(){

    /*
    TODO 20. Declara un array de tipo [f32, 3] simulando la posición de un modelo 3D [1.5, 0.0, -1.5]. Imprímel
     */

    let modelo:[f32;3]=[1.5,0.0,-1.5];

    println!("{:?}",modelo);

}

pub fn ejercicio_21(){

    /*
    TODO 21. Crea un array que dependa de la inferencia del compilador (sin poner los tipos): let numeros = [10, 20, 30] e imprímelo.
     */

    let numeros =[10,20,30];

    println!("{:?}",numeros);

}

pub fn ejercicio_22(){

    /*
    TODO 22. Declara un array de 5 booleanos inicializados aleatoriamente en true o false. Imprímelo en consola.
     */

    let boole:[bool;5]=[true,false,false,true,false];

    println!("{:?}",boole);

}

pub fn ejercicio_23(){

    /*
    TODO 23. Declara un array estricto de caracteres [char 4] con las letras 'A', 'B', 'C', 'D'. Imprímelo.
     */

    let carac:[char;4]=['A','B','C','D'];

    println!("{:?}",carac);

}

pub fn ejercicio_24(){

    /*
    TODO 24. Declara un array inmutable de 2 elementos tipo texto &str ("Usuario", "Admin") e imprímelo
     */

    let clases:[&str;2]=["Usuario","Admin"];

    println!("{:?}",clases);

}

// ! Regla 18: Acceso y Mutación de Arrays

pub fn ejercicio_25(){

    /*
    TODO 25. Declara un array de 4 números. Accede al primer elemento ([0]) e imprímelo usando {} estándar.
     */

    let numeros :[u8;4]=[13,23,31,50];

    println!("{}",numeros[0]);

}

pub fn ejercicio_26(){

    /*
    TODO 26. Declara un array con las notas de 3 exámenes. Accede al último elemento ([2]) e imprímelo.
     */

    let notas :[f32;3]=[3.1,5.0,2.0];

    println!("{}",notas[2]);

}

pub fn ejercicio_27(){

    /*
    TODO 27. Crea un array mutable de enteros mut puntajes = [10, 20, 30];. Cambia el valor del índice 1 a 50 y luego imprime el array completo.
     */
    let mut puntajes :[u8;3]=[10,20,30];

    println!("{:?}",puntajes);

    puntajes=[50,20,30];

    println!("{:?}",puntajes);

}

pub fn ejercicio_28(){

    /*
    TODO 28. Declara un array de 2 booleanos [true, false]. En un println!, evalúa e imprime si el índice 0 es diferente != del índice 1.
     */

    let booleanos:[bool;2]=[true,false];

    println!("{:?}",booleanos);

}

pub fn ejercicio_29(){

    /*
    TODO 29. Crea un array mutable de tres caracteres ['R', 'E', 'D']. Muta el primer índice para que sea una 'B' (cambiando la palabra a BED) e imprime el array.
    */

    let mut caracteres:[char;3]=['R','E','D'];

    println!("{:?}",caracteres);

    caracteres=['B','E','D'];

    println!("{:?}",caracteres);

}

pub fn ejercicio_30(){

    /*
    TODO 30. Declara un array de flotantes [10.5, 20.2]. Imprime el resultado de restar el índice 0 al índice 1.
     */

    let flotantes :[f32;2]=[10.5,20.2];

    println!("{:?}",flotantes[0]-flotantes[1]);

}

// ! Regla 19: Inicialización Repetida de Arrays

pub fn ejercicio_31(){

    /*
    TODO 31. Declara un array de 10 posiciones, todas inicializadas en el número 0. Imprímelo usando {:?}.
     */

    let repetir:[u8;10]=[0;10];

    println!("{:?}",repetir);

}

pub fn ejercicio_32(){

    /*
    TODO 32. Crea un buffer para una máscara de red: un array de 4 posiciones repetidas con el valor 255
     */

    let buffer:[u8;4]=[255;4];

}

pub fn ejercicio_33() {

    /*
    TODO 33. Declara un array que represente 50 celdas lógicas inicializadas todas en false. Usa el método .len() para imprimir el tamaño del array (no el contenido).
     */

    let celdas_logicas:[bool;50]=[false;50];

    println!("{:?}",celdas_logicas.len());

}

pub fn ejercicio_34(){

    /*
    TODO 34. Crea un array de 12 posiciones repetitivas con el flotante 1.0 de tipo f32. (declararlo explícitamente). Imprime el tamaño del array.
     */

    let pos_repetidas:[f32;12]=[1.0;12];

    println!("{:?}",pos_repetidas.len());

}

pub fn ejercicio_35(){

    /*
    TODO 35. Declara un array repetitivo con 5 caracteres 'X'. Imprímelo en consola para ver cómo se visualiza ['X', 'X', ...].
     */

    let rep_caracter:[char;5]=['X','X','X','X','X'];

    //let rep_caracter1:[char;5]=['X';5];

    println!("{:?}",rep_caracter);

}

pub fn ejercicio_36(){

    /*
    TODO 36. Crea un array mutable repetitivo de 3 posiciones inicializadas en 0. Muta exclusivamente la posición [1] a 100 y luego imprime todo el array.
     */

    let mut repetitivo:[u8;3]=[0;3];

    println!("{:?}",repetitivo);

    repetitivo=[0,100,0];

    println!("{:?}",repetitivo);

}

// ! Regla 20: Tipo "Unidad"

pub fn ejercicio_37(){

    /*
    TODO 37. Declara una variable llamada nada y asígnale explícitamente el tipo unidad () y el valor (). Imprímela con {:?}.
*/

    let nada:()=();

    println!("{:?}",nada);

}

pub fn ejercicio_38(){

    /*
    TODO 38. Crea una tupla regular que contenga un entero, y como segundo elemento, un Tipo Unidad: (10, ()). Imprímela.
    */

    let tupla:(u8,())=(10,());

    println!("{:?}",tupla);

}

pub fn ejercicio_39(){

    /*
    TODO 39. Asigna el resultado de una macro de impresión a una variable (ej. let resultado_print = println!("Hola");). Luego, imprime resultado_print con {:?} para comprobar que Rust devuelve () silenciosamente.
    */

    let resultado=println!("Hola"); // El tipo de dato es () automaticamente

    println!("{:?}",resultado);

}

pub fn ejercicio_40(){

    /*
    TODO 40. Declara un array de tamaño 3, donde cada elemento sea un Tipo Unidad: [(); 3]. Imprímelo.lto en tu código (sin asignarlo a nada) y añade un comentario documentando que esto no consume memoria y es evaluado a "nada". Compila y ejecuta.
     */

    let unidad:[();3]=[();3];
    // info se decalra un array de 3 elementos, cada elemento del array es  () un tipo Unidad   (VOID) esto significa que este Array contiene 3 Nnada que Ocupan Nada de Memoria.
        //info por que '()' No consume anda en memoria.

    println!("{:?}",unidad);

}

pub fn ejercicio_41(){

    /*
    TODO 41. Usa destructuración para extraer un Tipo Unidad de una tupla: let (a, _) = (50, ()) e imprime la variable a.
     */

    let tupla:(u8,())=(50,());

    let (a,())=tupla;

    println!("{a}");

}

pub fn ejercicio_42(){

    /*
    TODO 42. Escribe una línea solitaria con un valor Unidad () suelto en tu código (sin asignarlo a nada) y añade un comentario documentando que esto no consume memoria y es evaluado a "nada". Compila y ejecuta.
    */

    () //Esto no consume memoria y es evaluado en "nada";

}

pub fn ejercicio_43(){

    /*
    TODO  43. Declara una tupla que represente un "Clan" de Clash of Clans, conteniendo el nombre (&str) y un array mutable de 3 posiciones [u32 3] para almacenar la recolección de oro de tres días. Muta el segundo día del array dentro de la tupla e imprime la estructura completa.
     */

    let mut clan:(&str,[u32;3])=("A.V.C",[7500000,50000,125000000]);

    clan=("A.V.C",[7500000,5000000,125000000]);

    println!("{:?}",clan);

}

pub fn ejercicio_44(){

    /*
    TODO 44. Crea una tupla que contenga un identificador de paciente de una clínica ornitológica (u32) y un array de 2 elementos [f32 2] con el peso del ave antes y después del tratamiento. Accede al peso final en el array e imprímelo.
     */

    let identificador:(u32,[f32;2])=(1000000000,[1.25,2.5]);

    println!("{:?}",identificador.1[1]);

    }

pub fn ejercicio_45(){

    /*
     TODO 45. Declara una tupla que agrupe un nombre de sensor y un array de 4 booleanos de estado. Modifica el tercer estado del array a true y muestra la tupla con {:?}.
    */

    let mut dispositivo:(bool,bool,bool,bool)=(false,false,false,false);

    dispositivo.2=true;

    println!("{:?}",dispositivo);

}

pub fn ejercicio_46(){
    /*
     TODO 46. Simula una fila de una hoja de Excel: crea una tupla con un encabezado de columna y un array de 3 enteros. Suma los valores del array y muestra el resultado junto al nombre de la columna.
    .*/

    let fila_excel:(&str,[u8;3])=("edades",[19,27,49]);

    let suma:u8=fila_excel.1[0]+fila_excel.1[1]+fila_excel.1[2];

    println!("Suma de {:?}: {:?}",fila_excel.0,suma);

}

pub fn ejercicio_47(){

    /*
    TODO 47. Declara una tupla mutable que guarde un código de error y un array de 2 caracteres. Cambia el primer carácter del array y el código de la tupla, luego imprime el resultado.
     */

    let mut lista:(&str,[char;2])=("q1w2e3",['A','A']);

    lista.0="q2w2e3";
    lista.1[0]='B';

    println!("{:?}",lista);

}

pub fn ejercicio_48(){

    /*
    TODO 48. Crea una tupla con un nombre de experimento de métodos numéricos y un array de 3 flotantes f64. Muta el primer valor del array multiplicándolo por 2.0 y muestra la tupla
     */

    let mut experimento_metodos_numericos:(&str,[f64;3])=("Agente Naranja",[1.3656851656,313.13532,1.365681646]);

    experimento_metodos_numericos.1[0]=experimento_metodos_numericos.1[0]*2.0;

    println!("{:?}",experimento_metodos_numericos);

}

pub fn ejercicio_49(){

    /*
    TODO 49. Declara un array de 2 elementos, donde cada elemento sea una tupla (i32, i32). Accede al primer elemento del array, destrúcturalo en variables x e y, e imprime su suma.
    */

    let tuplas_inseption:[(i32,i32);2]=[(10,3),(12,2)];

    println!("primer elemento? {:?}",tuplas_inseption[0].1);

    let x:i32=tuplas_inseption[0].0;

    let y:i32=tuplas_inseption[0].1;

    println!("{:?}",x+y);

}

pub fn ejercicio_50(){

    /*
    TODO 50. Crea una tupla que contenga un nombre de ave y un array de 3 medidas de envergadura. Destructura la tupla para obtener el nombre y el array por separado. Imprime el array.
     */

    let ave:(&str,[u8;3])=("Aura Tiñosa",[65,81,183]);

    let ave_nombre:&str=ave.0;
    let ave_envergadura:[u8;3]=ave.1;

    println!("Nombre del Ave:{:?} \nEnvergadura del Ave:{:?}",ave_nombre,ave_envergadura);

}

pub fn ejercicio_51(){

    /*
    TODO 51. Declara una tupla con un ID de usuario y un array de 2 permisos [bool 2]. Usa destructuración para extraer el ID y el array. Luego, imprime si el primer permiso del array es true.
     */

    let usuario:(&str,[bool;2])=("10417452856396",[true,true]);

    let id_usuario:&str=usuario.0;
    let permisos_usuario:[bool;2]=usuario.1;

    println!("permiso usuario? {:?}",permisos_usuario[0]);

}

pub fn ejercicio_52(){

    /*
    TODO 52. Simula una respuesta de base de datos: una tupla con un booleano de éxito y un array de 4 bytes u8. Destructura la tupla e imprime los bytes usando el formato de depuración.
     */

    let ans_db:(bool,[u8;4])=(true,[255,255,125,078]);

    let ans_bool:bool=ans_db.0;
    let andarr:[u8;4]=ans_db.1;

}

pub fn ejercicio_53(){

    /*
    TODO 53. Declara una tupla que represente una fecha (u16, [u8 3]) (año y un array con día, mes, hora). Destructura la tupla y muestra el año y el mes extraídos del array.
     */

    let fecha:(u16,[u8;3])=(2026,[25,8,11]);

    let año:u16=fecha.0;
    let mes:[u8;3]=fecha.1;

    println!("año y mes de publicación {:?}/{:?}",año,mes[1]);

}

pub fn ejercicio_54(){

    /*
    TODO 54. Crea una tupla con un nivel de prioridad (char) y un array de 2 flotantes. Destructura ambos valores y muestra una oración combinándolos.
     */

    let prioridad:(char,[f32;2])=('Z',[1.315,9.999]);

    let nivel_prioridad:char=prioridad.0;
    let cola:[f32;2]=prioridad.1;

    println!("Te encuentras en el puesto {:?} de {:?} y tu prioridad es {nivel_prioridad}",cola[0],cola[1]);

}

// Info Regla R16 (Acceso Directo por Índice en Tuplas) + Regla R19 (Inicialización Repetitiva de Arrays)

pub fn ejercicio_55(){
    /*
    TODO 55. Declara una tupla que contenga un nombre de tabla de Excel y un array inicializado repetitivamente con 10 ceros [0; 10]. Accede al array usando el índice .1 de la tupla e imprime su longitud.
     */

    let excel :(&str,[u8;10])= ("Tienda",[0;10]);// 0 repetido 10 veces

}

pub fn ejercicio_56() {

    /*
    TODO 56. Crea una tupla que guarde un estado de carga y un array de 1024 bytes inicializados en 0. Usa el acceso por punto (ej. .1) para modificar el primer byte del array a 1.
     */

    let mut estado_carga:(bool,[u16;1024])= (false,[0;1024]);

    estado_carga.1[0] =1;

}

pub fn ejercicio_57(){

    /*
    TODO 57. Declara una tupla con un valor constante f64 y un array de 5 booleanos inicializados en true. Accede al valor constante mediante .0 y multiplícalo por la longitud del array accedido por .1.
     */

    let mut valor_constante:(f64,[bool;5])=(3.1415,[true;5]);

    valor_constante.0=valor_constante.0*valor_constante.1.len() as f64;

    println!("???: {:?}",valor_constante);

}

pub fn ejercicio_58() {
    /*
    TODO 58. Simula un registro de producción: una tupla con un identificador y un array de 100 flotantes inicializados en 1.0. Accede al array y cambia el valor de su última posición.
     */

    let mut registro_produccion: (u128, [f32; 100]) = (3656458646, [1.0; 100]);

    registro_produccion.1[99]=0.0;

}

pub fn ejercicio_59(){

    /*
    TODO 59. Declara una tupla mutable. En el índice .0 pon un carácter y en el .1 un array de 4 posiciones con el valor '?'. Cambia el carácter del índice .0 e imprime la tupla.
     */

    let mut nose :(char,[char;4])=('º',['?';4]);

    nose.0='ª';

}

pub fn ejercicio_60(){

    /*
    TODO 60. Crea una tupla que contenga un tipo de dato y un array de 8 posiciones inicializado en false. Usa el acceso por índice de la tupla para imprimir solo el array.
     */

    let incognita:(u8,[bool;8])=(128,[false;8]);

    println!("{:?}",incognita.1);

}

//Info Regla R20 (El Tipo Unidad ()) + Regla R14 (Tuplas Básicas) o R17 (Arrays)

pub fn ejercicio_61(){

    /*
    TODO 61. Declara una tupla que contenga un identificador de proceso, un booleano y el Tipo Unidad (). Imprímela para observar cómo se representa el () dentro de una estructura heterogénea.
     */

    let hetereogenea:(u8,bool,())=(18,false,());

    println!("{:?}",hetereogenea);

}

pub fn ejercicio_62(){

    /*
    TODO 62. Crea un array de 3 elementos donde cada elemento sea una tupla que contenga un número y un Tipo Unidad (i32, ()). Imprime el array completo.
     */

    let array:[(u8,());3]=[(12,()),(14,()),(16,())];

    println!("{:?}",array);

}

pub fn ejercicio_63(){

    /*
    TODO 63. Declara una tupla que agrupe un array de 2 enteros y un Tipo Unidad. Accede al Tipo Unidad mediante su índice de tupla e imprímelo.
     */

    let agrupar:([u8;2],())=([12,13],());

}

pub fn ejercicio_64(){

    /*
    TODO 64. Simula una función de Excel que no devuelve valor: crea una tupla con el nombre de la función "GUARDAR" y un Tipo Unidad. Destructura la tupla e imprime solo el nombre.
     */

    let funcion:(&str,())=("GUARDAR",());

    let (nombre,b)=funcion;

    println!("{}",nombre);

}

pub fn ejercicio_65(){

    /*
    TODO 65. Declara un array mutable de 2 tipos unidad [() 2]. Intenta asignar el valor de una posición a la otra e imprime el array.
     */

    let mut arr:[();2]=[();2];

    arr[1]=arr[0];

    println!("{:?} {:?}",arr[0],arr[1]);

}

pub fn ejercicio_66(){

    /*
    TODO 66. Crea una tupla anidada donde el elemento más interno sea un Tipo Unidad: (1, (true, ())). Accede al Tipo Unidad usando índices encadenados y muéstralo.
     */

    let tupla_anidada:(u8,(bool,()))=(1,(true,()));

    println!("{:?}",tupla_anidada.1.1);

}

// ! Dificultad Elevada
// Info Regla R14 (Tuplas) + Regla R15 (Destructuración) + Regla R17 (Arrays)

pub fn ejercicio_67(){

    /*
    TODO// 67. Declara una tupla que contenga un array de 3 enteros [i32 3] y un flotante f64. Destructura la tupla en dos variables (mediciones y promedio) e imprime la variable del array utilizando el formateador de depuración {:?}.
   */

    let tupla:([i32;3],f64)=([14,16,18],3.141516);

    let (mediciones,promedio)=tupla;

    println!("{:?}",mediciones);

}

pub fn ejercicio_68(){

    /*
    TODO 68. Crea un array de 2 elementos, donde cada elemento sea una tupla (&str, u32). Accede al primer elemento del array ([0]), destrúcturalo en nombre y edad, e imprímelos en una oración completa.
     */

  let array:[(&str,u32);2]=[("Juan",32),("Camilo",16)];



}

pub fn ejercicio_69(){

    /*
    TODO 69. Declara una tupla que represente a un estudiante universitario: su ID (u32) y un array con sus 3 notas parciales [f32 3]. Destructura la tupla, calcula matemáticamente el promedio sumando los índices del array extraído, e imprime el resultado final.
    */

    let estudiante_universitario:(u32,[f32;3])=(1234,[1.5,2.0,4.5]);

    let (a,b)=estudiante_universitario;
    let promedio:f32=b[0]+b[1]+b[2];

    println!("Promedio: {}",promedio)

}

pub fn ejercicio_70(){
    /*
    TODO 70. Crea una tupla que agrupe un booleano de estado y un array estricto de 4 caracteres [char 4]. Usa destructuración para aislar el array e imprímelo en consola completo para verificar su contenido.
     */

    let tupla_0:(bool,[char;4])=(true,['P','I','Y','R']);

    let (a,b)=tupla_0;

    println!("{:?}", b);

}

pub fn ejercicio_71(){

    /*
    TODO 71. Declara un array que contenga 3 tuplas idénticas en tipo: [(i32, f64) 3]. Accede al índice 1 del array, destructura esa tupla específica en x e y, y multiplica ambos valores imprimiendo el resultado.
     */

    let array :[(i32,f64);3]=[(10,1.0),(20,2.0),(30,3.0)];

    let (x,y)=array[0];

    println!("Resulatdo: {:?}",x as f64*y);

}

pub fn ejercicio_72(){

    /*
    TODO 72. Crea una estructura anidada: una tupla que contenga un array de 2 booleanos y un número u8. Destructura la tupla principal para extraer el array y luego imprime el valor del primer booleano de ese array extraído.
     */

    let anidada:([bool;2],u8)=([true,false],128);

    let (a,b)=anidada;

    println!("{:?}",a[0]);

}

// Regla R16 (Indexación de Tuplas) + Regla R18 (Mutación de Arrays) + Regla R19 (Inicialización Repetitiva)

pub fn ejercicio_73(){
    /*
    TODO 73. Declara una tupla mutable que contenga un texto "Buffer" y un array inicializado repetitivamente con 5 ceros [0 5]. Accede al array a través del índice de la tupla (.1), muta su tercer elemento (índice 2) asignándole el valor 100, e imprime la tupla completa.
    */

    let mut tupla_mutable:(&str,[u8;5])=("Buffer",[0;5]);

    tupla_mutable.1[2]=100;

    println!("{:?}",tupla_mutable);

}

pub fn ejercicio_74(){

    /*
    TODO 74. Crea un array mutable de 3 tuplas inicializadas repetitivamente: [(0, 0) 3]. Accede a la primera tupla del array ([0]) y usa la indexación de tupla (.1) para mutar el segundo valor a 50. Imprime el array.
     */

    let mut array:[(u8,u8);3]=[(0,0),(1,1),(2,2)];

    array[(0)].1=50;

    println!("{:?}",array);

}

pub fn ejercicio_75(){

    /*
    TODO 75. Declara una tupla mutable que albergue un array repetitivo de 10 booleanos en false. Usa el acceso por punto (.0) para llegar al array, muta el último elemento ([9]) a true, e imprime el array usando la tupla.
    */

    let mut repetitivo:(bool,bool,bool,bool,bool,bool,bool,bool,bool,bool)=(true,true,true,true,true,true,true,false,false,false);

    repetitivo.9=true;

    println!("{:?}",repetitivo);

}

pub fn ejercicio_76(){

    /*
    TODO 76. Simula un puntero dinámico: crea una tupla mutable con un array repetitivo [1.0f32 4] en la posición 0, y un índice usize (ej. 2) en la posición 1. Usa el valor de .1 para acceder dinámicamente al array en .0 y mutar ese elemento a 99.9. Imprime la tupla.
    */

    let mut puntero:([f32;4])=([0.0,1.0,2.0,3.0]);

    puntero[1]=99.9;

    println!("{:?}",puntero);

}

pub fn ejercicio_77(){

    /*
    TODO 77. Declara una tupla que contenga dos arrays repetitivos: ([0 5], [1 5]). Muta el índice 4 del primer array (accediendo con .0) y el índice 0 del segundo array (accediendo con .1). Imprime la estructura resultante.
     */

    let mut array:([u8;5],[u8;5])=([0;5],[1;5]);

    array.0[4]=99;

    array.1[0]=89;

    println!("{:?}",array);

}

pub fn ejercicio_78(){

    /*
    TODO 78. Crea una tupla con un array repetitivo de 8 caracteres guion ['-' 8]. Accede al array mediante la indexación de tupla y cambia el carácter del índice 4 por una 'X' (simulando un impacto en un radar). Imprime la tupla.
     */

    let mut array1:([char;8])=(['-';8]);

    array1[4]='X';

    println!("{:?}",array1);

}

//! Dificultad Heroica
//Info Regla R14 (Tuplas Básicas) + Regla R15 (Destructuración) + Regla R17 (Arrays) + Regla R18 (Mutación de Arrays)

pub fn ejercicio_79({

    /*
    TODO 85. Declara una tupla mutable que contenga un ID (u32) y un array de 3 puntajes [u16 3]. Accede al array a través de la tupla (usando .1) para mutar el segundo puntaje (índice 1). Luego, destructura la tupla completa en las variables id y puntajes e imprime el nuevo valor del array usando la variable extraída.
     */

    let mut tupla:(u32,[u16;3])=(1000,[1,2,3]);

    tupla.1[1]=5;

    let (id,puntajes)=tupla;

    println!("{puntajs}");

}

pub fn ejercicio_80(){

    /*
    TODO 86. Crea un array mutable de 2 elementos, donde cada elemento sea una tupla (i32, bool). Accede al primer elemento del array ([0]) y muta el booleano de la tupla (usando .1) a true. Luego, destructura esa misma tupla del array en numero y estado, e imprime ambas variables.
    */

    let mut array:[(i32, bool);2]=[(3,false);2];

    array[0].1=true;

    let array1=array[0];

    let array2=array[1];

    let (numero,estado)=array1;

    let (numero1,estado1)=array2;

    println!("{numero} | {numero1} | {estado} | {estado1}");

}

pub fn ejercicio_81(){

    /*
    TODO 87. Declara una tupla mutable con el nombre de un sensor (&str) y un array de 4 lecturas flotantes [f32 4]. Muta la última lectura del array accediendo directamente con índices encadenados (ej. tupla.1[3]). Aplica destructuración para separar el nombre del array, e imprime el array mutado.
88. Simula una matriz de coordenadas: declara un array de 3 posiciones donde cada celda es una tupla mutable (f64, f64). Muta el eje Y de la última tupla del array ([2].1). Destructura esa tupla específica en x e y e imprime la coordenada modificada.
89. Declara una tupla mutable que agrupe dos arrays: ([10, 20], [30, 40]). Muta el primer array (índice 0) usando .0[0] = 99. Destructura la tupla en array_a y array_b. Suma matemáticamente el primer elemento de array_a con el último de array_b e imprime el resultado.
90. Crea una tupla anidada mutable: (1, ([100, 200], true)). Accede a lo más profundo de la estructura usando .1.0[1] para mutar el valor 200 a 500. Luego, destructura la estructura completa en un solo paso (ej. let (id, (arreglo, flag)) = tupla;) e imprime el arreglo modificado.
     */



}