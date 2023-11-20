

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


    combinatoria(2, elementos);
}

fn combinatoria(n: usize, elementos: Vec<Vec<String>>) -> Vec<Vec<String>>{
    let mut combinaciones: Vec<Vec<String>> = Vec::new();

    let mut resultado: Vec<Vec<String>> = Vec::new();
    let mut combinacion_actual: Vec<String> = Vec::new();

    println!("Iniciando combinaciones de {} elementos", n);
    combinaciones_lista_de_listas(&elementos, n, &mut combinacion_actual, 0, &mut resultado);

    for combinacion in &resultado {
        println!("{:?}", combinacion);
        combinaciones.push(combinacion.clone());
    }

    combinaciones
}

fn combinaciones_lista_de_listas<T:Clone>(
        listas: &Vec<Vec<T>>,
        n: usize, 
        combinacion_actual: &mut Vec<T>, 
        index: usize,
        resultado: &mut Vec<Vec<T>>, 
    ){
    //Imprimimos todas las variables que entraron:
    println!("Index: {} | N: {}", index, n);


    //Si la combinacion actual tiene el tamaño que queremos, la agregamos al resultado
    if combinacion_actual.len() == n {
        resultado.push(combinacion_actual.clone());
        return;
    }

    //Si el index es igual al tamaño de la lista de elementos, no hay nada que hacer
    if index == listas.len() {
        return;
    }

    //Obtenemos la lista actual
    let lista_actual = &listas[index];


    //Iteramos sobre los elementos de la lista actual
    for elemento in lista_actual {
        combinacion_actual.push(elemento.clone());
        combinaciones_lista_de_listas(listas, n, combinacion_actual, index+1, resultado);
        combinacion_actual.pop();
    }

    //Llamamos a la funcion recursivamente con el siguiente index
    combinaciones_lista_de_listas(listas, n, combinacion_actual, index, resultado)
}