fn div(a: f64, b:f64) -> Result<f64, String> {
    if b == 0.0 {
        return Err(String::from("Erro em dividir"))
    }

    Ok(a/b)
}

fn main() {
    let div1 = div(5.0, 0.0);

    match div1 {
        Ok(calc) => println!("Result: {}", calc),
        Err(msg) => println!("Error: {}", msg)
    }
}
