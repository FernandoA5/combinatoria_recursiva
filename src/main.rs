//Llamamos al archivo combinatoria.rs
mod combinatoria;


fn main() {
    //{"Survived": ["0", "1"], "Pclass": ["2", "3", "1"], "Embarked": ["S", "C", "Q"]}
    //Declaramos las 3 Listas que usaremos durante el ejemplo
    let mut lista1: Vec<String> = Vec::new();
    let mut lista2: Vec<String> = Vec::new();
    let mut lista3: Vec<String> = Vec::new();

    //Agregamos los elementos a la lista1
    lista1.push("0".to_string());
    lista1.push("1".to_string());

    //Agregamos los elementos a la lista2
    lista2.push("2".to_string());
    lista2.push("3".to_string());
    lista2.push("1".to_string());

    //Agregamos los elementos a la lista3
    lista3.push("S".to_string());
    lista3.push("C".to_string());
    lista3.push("Q".to_string());

    let mut elementos: Vec<Vec<String>> = Vec::new();
    elementos.push(lista1.clone());
    elementos.push(lista2.clone());
    elementos.push(lista3.clone());

    println!("Lista 1: {:?}", lista1.clone());
    println!("Lista 2: {:?}", lista2.clone());
    println!("Lista 3: {:?}", lista3.clone());

    // let combinaciones = combinatoria(3, elementos);
    let combinaciones = combinatoria::combinatoria(3, elementos);
    for combinacion in combinaciones {
        println!("Combinacion: {:?}", combinacion);
    }
}




