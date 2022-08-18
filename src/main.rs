fn main() {
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
