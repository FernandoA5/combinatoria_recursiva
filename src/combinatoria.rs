pub fn combinatoria(n: usize, elementos: Vec<Vec<String>>) -> Vec<Vec<String>> {
    let mut combinaciones: Vec<Vec<String>> = Vec::new();
    combinatoria_recursiva(n, elementos, &mut combinaciones, Vec::new());
    combinaciones
}

fn combinatoria_recursiva(n: usize, elementos: Vec<Vec<String>>, combinaciones: &mut Vec<Vec<String>>, combinacion_actual: Vec<String>) {
    if n == 0 {
        combinaciones.push(combinacion_actual);
        return;
    }

    for i in 0..elementos.len() {
        for j in 0..elementos[i].len() {
            let mut nueva_combinacion = combinacion_actual.clone();
            nueva_combinacion.push(elementos[i][j].clone());
            let mut nuevos_elementos = elementos.clone();
            nuevos_elementos.remove(i);
            combinatoria_recursiva(n - 1, nuevos_elementos, combinaciones, nueva_combinacion);
        }
    }
}