//info Regla 1 (let) + Regla 4 (Enteros con Signo) + Regla 12 (print!)

pub fn ejercicio121(){

    /*
    TODO 121. Declara dos variables inmutables de tipo i16 simulando temperaturas bajo cero. Súmalas y muestra el resultado en una oración completa usando println!.
    //
     */

    let temperatura_media:i16=-8;

    let temperatura_local:i16=-20;

    print!("La temperatura es de : {}",temperatura_media+temperatura_local);
    println!();

}

pub fn ejercicio122(){

    /*
    TODO 122. Declara una deuda inmutable gigante usando i64 y un pago inmutable usando i32. Calcula el saldo restante directamente dentro de las llaves {} de un macro println!.
     */

    let deuda:i64=i64::MAX;
    let mut pago:i32=i32::MAX;

    println!("Saldo restante: {}",deuda-pago as i64)

}

pub fn ejerciciio123(){

    /*
    TODO 123. Usa una expresión matemática para declarar un let de tipo i128 que contenga el resultado de multiplicar un número negativo masivo por 2. Imprímelo usando println!.
     */

    let resultado :i128=-124145266643234533332*2;

    println!("{}",resultado)

}

pub fn ejercicio124(){

    /*
        TODO 124. Declara un año histórico (i32 negativo para Antes de Cristo) inmutable. En un println!, muestra el texto "El imperio cayó en el año: [variable]".
     */

    let born :i32=-132;

    println!("El imperio cayó en el año {}",born);

}

pub fn ejercicio125(){

    /*
    TODO 125. Declara una variable i8 inmutable a la que le asignes exactamente el límite inferior posible usando i8::MIN. Imprime un println! que advierta "Fondo matemático alcanzado: [variable]".
     */

    let var :i8 = i8::MIN;

    println!("Fondo matemático alanzado {var}");

}

pub fn ejercicio126(){

    /*
    TODO 126. Declara una coordenada submarina inmutable usando el sufijo de tipo (ej. -500i32). Usa println! para mostrar la profundidad formateada.
     */

    let coordenada:i32=-500i32;

    println!("Profundidad: {}",coordenada);

}

// Info Regla R2 (mut) + Regla R5 (Enteros sin Signo) + Regla R11 (print!)

pub fn ejercicio127(){

    /*
    TODO 127. Declara una variable mutable de tipo u8 en 0. Usa print! para mostrar "Progreso: 0%...", mútala a 50, y usa otro print! para añadir " 50%...".
     */

    let mut var:u8=0;

    println!("Progreso: {var}%...");

    var=50;

    println!("Progreos: {var}%...");
}

pub fn ejercicio128(){

    /*
    TODO 128. Declara un puerto de red como mut u16 inicializado en 80. Usa print! para indicar "Puerto [variable] cerrado -> ", cámbialo a 443, y usa un segundo print! para decir "Abriendo puerto [variable]".
     */

    let mut puerto_red:u16=80;

    println!("Puerto {puerto_red} cerrado ->");

    puerto_red=443;

    println!("Abriendo puerto {puerto_red}");

}

pub fn ejercicio129(){

    /*
    TODO 129. Simula el inventario de una tienda: declara un mut u32 con 100 artículos. Usa print! para mostrar el stock, réstale 5 artículos vendidos, y usa print! para mostrar el nuevo stock en la misma línea.
     */

    let mut articulos:u32=100;

    print!("Stock: {articulos}   ");

    articulos=95;

    println!("Stock: {articulos}");

}

pub fn ejercicio130(){

    /*
    TODO 130. Crea un ID de sesión mut u64. Imprímelo con print!, reasígnale un nuevo valor masivo y usa print! para indicar que la sesión fue actualizada, sin romper la línea de la consola.
     */

    let mut id:u64=130;
    print!("{id}  ");
    id=u64::MAX;
    print!("{id}");
}

pub fn ejercicio131(){

    /*
    TODO 131. Declara la población de un país como mut u128 usando guiones bajos para lectura (ej. 50_000_000). Muestra la cifra con print!, súmale 1 millón, e imprime la nueva cifra contigua.
     */

    let mut poblacion:u128=50_000_000;

    print!("{poblacion};");
    poblacion=poblacion+1_000_0000;
    print!("{poblacion}");
}

pub fn ejercicio132(){

    /*
    TODO 132. Declara un contador de vidas mut u8 (inicia en 3). Usa print! para mostrar corazones (ej. 3), réstale 1, y finaliza la línea con print! mostrando las vidas restantes.
     */

    let mut contador:u8=3;

    print!("Corazones {contador}");
    contador=contador-1;
    print!("Corazones {contador}");
}

// Info Regla R3 (Shadowing) + Regla R7 (f32) + Regla R8 (f64)

pub fn ejercicio133(){

    /*
    TODO 133. Declara una constante física inmutable como f32. Usa shadowing para "actualizarla" a f64 dándole más decimales de precisión, manteniendo el mismo nombre de variable.
     */

    let gravedad:f32=9.8;

    let gravedad:f64=9.81;
}

pub fn ejercicio134(){

    /*
    TODO 134. Declara un precio como texto ("19.99"). Usa shadowing para convertir ese mismo nombre de variable en un f32 (asignándole el número 19.99) y luego haz un segundo shadowing para transformarlo a f64.
     */

    let precio:&str="19.99";

    let precio:f32=19.99;

    let precio:f64=19.999;
}

pub fn ejercicio135(){

    /*
    TODO 135. Calcula la velocidad como f32 dividiendo dos números. Haz shadowing sobre la variable velocidad asumiendo un entorno de alta precisión, reasignándola como un cálculo f64.
     */

    let velocidad:f32=154.5/2455.6;

    let velocidad:f64=154.3/2455.6;
}

pub fn ejercicio136(){

    /*
    TODO 136. Abre un bloque de código { }. Fuera de él, declara la gravedad como f64 (9.80665). Dentro del bloque, haz shadowing para crear una versión menos precisa f32 (9.8)..
     */

    let gravedad :f64=9.80665;

    {
        let gravedad:f32=9.8;
    }
}

pub fn ejercicio137(){

    /*
    TODO 137. Declara el radio de un átomo en f32 usando notación científica. Usa shadowing para convertir esa medida a f64 multiplicándola por 2.0.
     */

    let radio_atomo:f32=12.5e-20;

    let radio_atomo:f64=radio_atomo as f64*2.0;
}

pub fn ejercicio138(){

    /*
    TODO 138. Usa shadowing en cascada: inicializa un valor en f32, hazle shadowing a f64 sumándole un decimal largo, y vuelve a hacerle shadowing a f32 recortando el valor
     */

    let valor:f32=1.0;

    let valor:f64=2.10123456789;

    let valor:f32=2.101234;

}

// Info Regla R6 (usize / isize) + Regla R9 (bool) + Regla R10 (char)

pub fn ejercicio139(){

    /*
    TODO 139. Declara un índice de array usize. Declara un char 'X' representando un objetivo. Crea un booleano que evalúe si el índice es mayor a 0 para saber si la búsqueda ya inició.
     */

    let indice:usize=0;

    let objetivo:char='X';

    let eva:bool= indice > 0;
}

pub fn ejercicio140(){

    /*
    TODO 140. Declara un desplazamiento de puntero isize negativo. Crea un booleano que evalúe si el desplazamiento es menor a 0, y si lo es, representa el estado con un carácter 'R' (Retroceso).
     */


    // # BLANK

}

pub fn ejercicio141(){

    /*
    TODO 141. Simula el estado de un bucle: un usize para la iteración actual (ej. 5), un booleano en false indicando que no ha terminado, y un char de espera (ej. '⏳').
	142. Asigna la memoria máxima a un usize usando usize::MAX. Crea un booleano que compruebe si esa variable es igual a sí misma (==). Acompaña con un carácter 'M' (Máximo).
	143. Un sistema de juego en cuadrícula: un desplazamiento vertical de isize, un estado lógico booleano de "colisión", y el símbolo del muro como char ('#').
	144. Declara un índice base en usize inferido con sufijo (ej. 10usize), un booleano que indique si el índice es par (matemáticamente simulado), y una letra 'P' para categorizarlo.
     */



}