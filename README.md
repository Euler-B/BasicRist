# Variables en Rust
Las variables en Rust son fundamentales para almacenar y manipular datos. A continuación, se detallan las características más relevantes sobre su uso y comportamiento.
### Inmutabilidad por Defecto
Por defecto, las variables en Rust son inmutables. Esto significa que una vez que un valor está asignado a una variable, no se puede cambiar. Esta característica promueve la seguridad y la concurrencia en el código, ya que reduce el riesgo de errores relacionados con la modificación accidental de valores.
Ejemplo de variable inmutable:
```rust
fn main() {
    let x = 5;
    println!("El valor de x es: {}", x);
    // x = 6; // Esto causará un error de compilación
}
```

### Mutabilidad
Si se desea que una variable sea mutable, se debe utilizar la palabra clave mut al declararla. Esto permite que su valor sea modificado después de la asignación inicia.
Ejemplo de variable mutable:
```rust
fn main() {
    let mut x = 5;
    println!("El valor de x es: {}", x);
    x = 6; // Ahora es posible cambiar el valor
    println!("El nuevo valor de x es: {}", x);
}
```
### Tipos de Datos
Las variables pueden estar asociadas a diferentes tipos de datos, como enteros, flotantes, booleanos y cadenas. Rust permite la inferencia de tipos, pero también se puede especificar el tipo explícitamente.
Ejemplo de declaración con tipo explícito:
```rust
let numero: i32 = 42; // Declaración explícita del tipo
```

### Constantes
Las constantes son similares a las variables, pero su valor no puede cambiar una vez asignado. Se declaran con la palabra clave const y deben tener un tipo definido.
Ejemplo de constante:
```rust
const TRES_HORAS_EN_SEGUNDOS: u32 = 60 * 60 * 3;
```

### Shadowing
El shadowing permite declarar una nueva variable con el mismo nombre que una anterior. Esto no modifica la variable original, sino que crea una nueva en el mismo ámbito.
Ejemplo de shadowing:
```rust
fn main() {
    let x = 5;
    let x = x + 1; // Se crea una nueva variable 'x'
    {
        let x = x * 2; // Se crea otra 'x' en un nuevo ámbito
        println!("El valor de x en el alcance interno es: {}", x);
    }
    println!("El valor de x es: {}", x); // Se refiere a la 'x' externa
}
```

