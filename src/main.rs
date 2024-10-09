fn main() {
    // Obtengo el nombre del usuario
    println!("Hola, Buenas tardes cual es tu nombre:");
    let mut nombre: String = String::new();
    std::io::stdin().read_line(&mut nombre).unwrap();
    nombre = nombre.trim().to_string();

    // Voy  a obtener la edad del usuario
    println!("Ahora me podrias decir cual es tu año de nacimiento:");
    let mut año_nacimiento: String = String::new();
    std::io::stdin().read_line(&mut año_nacimiento).unwrap();
    let año_nacimiento_int: i32 = año_nacimiento.trim().parse().unwrap();

    //convierto la edad en entero
    let año_actual = 2024;
    let edad = año_actual - año_nacimiento_int;

    //Muestro la informacion en base a los datos que he recibido del usuario
    println!("Hola Sr. {}, usted tiene {} años de edad", nombre, edad);

    if año_nacimiento_int <= 2006 {
        println!("Ademas es de la Generacion Z");
    } else if año_nacimiento_int <= 1996 {
        println!("Ademas es de la Generacion Y");
    } else if año_nacimiento_int <= 1980 {
        println!("Ademas es de la Generacion X");
    } else {
        println!("Ademas es de la Generacion Baby Boomer");
    }
}
