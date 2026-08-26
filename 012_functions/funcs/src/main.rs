fn main() {
    let a: i32 = 0;
    let b: i32 = 10;

    println!("Soma de {0} + {1} = {2}", a, b, soma(a, b));
    println!("Subtração de {0} - {1} = {2}", a, b, subtracao(a, b));
    println!("Multiplicação de {0} * {1} = {2}", a, b, multi(a, b));
    println!("Divisão de {0} / {1} = {2}", a, b, dividir(a as f64, b as f64));
}

fn soma(a :i32, b :i32) -> i32 {
    return a + b;
}

fn subtracao(a :i32, b :i32) -> i32 {
    return a - b;
}

fn multi(a :i32, b :i32) -> i32 {
    return a * b;
}

fn dividir(a: f64, b: f64) -> f64 {
    if a == 0.0 || b == 0.0 {
        return 0.0;
    }

    return a / b;
}