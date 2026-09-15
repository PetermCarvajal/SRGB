use std::io;

pub fn facil(){

    // info Regla 1:Formato posicional y Nombrado

    /*
    TODO 1.Declara 3 variables de texto. Usa println! e índices posicionales ({2}, {1}, {0}) para imprimirlas en orden inverso al que las pasaste como argumentos.
     */

    pub fn ejercicio_1(){

        let mut input1:String=String::new();
        let mut input2:String=String::new();
        let mut input3:String=String::new();

        io::stdin().read_line(&mut input1).expect("Fallo al leer");
        io::stdin().read_line(&mut input2).expect("Fallo al leer 2");
        io::stdin().read_line(&mut input3).expect("Fallo al leer 3");

        println!("útimo: {2}\nSegundo: {1}\nPrimero: {}",input1,input2,input3);
    }

    pub fn ejercicio_2(){

        /*
        TODO 2.Imprime la frase "Un [perro] no es un [gato], pero un [perro] es leal" pasando solo dos argumentos a la macro y repitiendo los índices.
         */

        let mut perro:String=String::new();
        let mut gato:String=String::new();

        io::stdin().read_line(&mut perro).expect("Fallo al Wao");
        io::stdin().read_line(&mut gato).expect("Fallo el Miao");

        println!("Un {0} no es un {1}, pero un {0} es leal",perro,gato);

    }

    pub fn ejercicio_3(){

        /*
        TODO 3.Usa formato nombrado: println!("{sujeto} tiene {edad}", sujeto="Juan", edad=30);.
         */

        println!("{sujeto} tiene {edad}",sujeto="Juan",edad=30);

    }

    pub fn ejercicio_4(){

        /*
        TODO 4.Combina posicional y nombrado: pasa un argumento posicional {0} y otro nombrado {sistema="Rust"} en el mismo println!.
         */

        println!("argumento posicional {0} pero también es argumento nombrado {sistema}",sistema="Rust");

    }

    pub fn ejercicio_5(){

        /*
        TODO 5.Declara dos números a = 10 y b = 20. Usa formato posicional para imprimir la ecuación "20 + 10 = 30" (calcula la suma en el último argumento).
         */

        let a:u8 = 10;
        let b:u8 =20;

        print!("{1}+{0}={2}",a,b,a+b);

    }

    pub fn ejercicio_6(){

        /*
        TODO 6.Imprime una tabla de 2 columnas simple usando formato posicional para garantizar que el argumento 0 siempre esté a la izquierda y el 1 a la derecha en múltiples líneas.
         */

        println!("{0}{1}\n{0}{1}",a=0,b=1);

    }

    // info Regla 2:Formato Avanzado (Padding, Alineación y Precisión)

    /*
    info
        Padding {:#>#} {:#<#}
        Alineación {:^}  ^ Permite Centrar
        Rellenar {:()>#} () está posición permite rellenar de  de ese caracter
     */

    pub fn ejercicio_7(){

    /*
    TODO 7.Imprime el número 42 forzándolo a tener 5 dígitos, rellenando con ceros a la izquierda ({:0>5}).
     */

        let a:u8=42;

        println!("{:0>5}",a);

    }

    pub fn ejercicio_8(){

        /*
        TODO 8.Imprime el número flotante 3.14159265 forzándolo a mostrar solo 3 decimales ({:.3}).
         */

        let pi:f32=3.14159265;

        println!("{:.3}",pi);

    }

    pub fn ejercicio_9(){

        /*
        TODO 9.Declara un número 7. Alínealo a la derecha usando espacios en blanco ocupando un ancho total de 10 caracteres ({:>10}).
         */

        let num:u8=7;

        println!("{:>10}",num);

    }

    pub fn ejercicio_10(){

        /*
        TODO 10.Imprime un texto corto "Rust" alineado a la izquierda dentro de un bloque de 15 caracteres de ancho, llenando el resto con guiones ({:-<15}).
         */

        let txt:&str="Rust";

        println!("{:-15}",txt);

    }

    pub fn ejercicio_11(){

        /*
        TODO 11.Centra la letra 'X' en un espacio de 11 caracteres rodeada de asteriscos ({:*^11}).
         */

        let char:char='X';

        println!("{:*^11}",char);

    }

    pub fn ejercicio_12(){

        /*
        TODO 12.Formatea un entero grande 1000000 combinando padding de ceros a la izquierda y un ancho de 10.
         */

        let entero:u32=1000000;

        println!("{:0^10}",entero);

    }

    // Info Regla 3: Importación standar y Buffer Dinamico

    pub fn ejercicio_13(){

        /*
        TODO 13.Escribe un bloque de código que importe el módulo io y declare una variable mut buffer = String::new();. Imprime la capacidad actual del buffer en bytes (buffer.capacity()).
         */

        let mut buffer:String = String::new();

        println!("{}",buffer.capacity());

    }

    pub fn ejercicio_14(){

        /*
        TODO 14.Importa std::io. Crea un String::new() y usa la macro print! (sin salto) para indicarle al usuario que la memoria está preparada.
         */

        let mut memoria:String=String::new();

        print!("");

    }

    pub fn ejercicio_15(){

        /*
        TODO 15.Declara dos buffers diferentes de tipo String::new() (ej. input1 e input2). Imprime que ambos están vacíos evaluando .is_empty().
         */

        let mut buffer:String=String::new();
        let mut buffer1:String=String::new();

        println!("{},{}",buffer.is_empty(),buffer1.is_empty());

    }

    pub fn ejercicio_16(){

        /*
        TODO 16.Omite importar std::io al inicio del archivo y usa la ruta absoluta directamente en el código para inicializar memoria temporal y un input simulado (solo preparación, usa std::io::stdin).
         */

        let mut temp:String=String::new();

    std::io::stdin().read_line(&mut temp).expect("Mission Failed");

    }

    pub fn ejercicio_17(){

        /*
        TODO 17.Crea un String::new() y, aunque está vacío, imprímelo en pantalla dentro de unos corchetes [{}] para confirmar su vacuidad.
         */

        let vacio:String=String::new();

        println!("[{}]",vacio);

    }

    pub fn ejercicio_18(){

        /*
        TODO 18.Instancia un mut buffer_entrada = String::new();. Comprueba su longitud en caracteres (.len()) e imprímela.
         */

        let mut buffer_entrada:String=String::new();

        println!("{}",buffer_entrada.len());

    }

    // Info Regla 4: Captura de entrada (read_line)

    pub fn ejercicio_19(){

        /*
        TODO 19.Solicita al usuario su nombre con print!. Captura la entrada en un String usando io::stdin().read_line(&mut variable). Imprime el texto capturado.
         */

        let mut variable:String=String::new();

        println!("Porfavor Ingrese su nombre");
        io::stdin().read_line(&mut variable).expect("Error");

        println!("Bienvenido");

    }

    pub fn ejercicio_20(){

        /*
       TODO 20.Declara un buffer. Captura lo que escriba el usuario y luego imprime la cantidad de bytes que capturó el método (usando el valor de retorno que es un Result<usize>).
         */

        let mut buffer:String=String::new();

        println!("Escriba");

        io::stdin().read_line(&mut buffer).expect("Error");

            println!("{:?}",buffer.bytes());

    }

    pub fn ejercicio_21(){

        /*
        TODO 21.Solicita una palabra. Captúrala y vuelve a imprimirla pero rodeada de comillas simples para que observes el salto de línea \n oculto que el usuario introdujo al dar Enter.
        */

        println!("Ingrese una palabra");

        let mut palabra:String=String::new();
        io::stdin().read_line(&mut palabra).expect("Error");

    }
    
}