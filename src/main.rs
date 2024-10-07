fn main() {
    // Declaramos 2 numeros que vamos a sumar 
    let numero_1 = 123;
    let numero_2 = 321;

    let suma = numero_1 + numero_2;

    loop {
    // Mostrar los dos numeros en pantalla 
        println!("Por favor escribir la suma de {} y {}: ", numero_1, numero_2);

    // Obtener del usuario el numero que representa la suma.
        let mut suma_usuario = String::new();
        std::io::stdin().read_line(&mut suma_usuario).unwrap();
    
        let suma_usuario_int : i32 = suma_usuario.trim().parse().unwrap();

        if suma_usuario_int == suma {
            println!("Lo has hecho muy bien, el resultado {} es correcto", suma);
            break;
        } else {
            println!("El resultado {} no es correcto, por favor intentalo de nuevo", suma_usuario_int);
        }

    // Obtengo el nombre del usuario
    println!("Hola, Buenas tardes cual es tu nombre:");
    let mut nombre: String = String::new();
    std::io::stdin().read_line(&mut nombre).unwrap();
    nombre = nombre.trim().to_string();

    // Voy  a obtener la edad del usuario
    println!("Ahora me podrias decir cual es tu edad:");
    let mut edad: String = String::new();
    std::io::stdin().read_line(&mut edad).unwrap();

    //convierto la edad en entero
    let  edad_int: u8 = edad.trim().parse().unwrap();

    //Muestro la informacion en base a los datos que he recibido del usuario
    println!("Hola Sr. {}, usted tiene {} años de edad", nombre, edad_int);
}
