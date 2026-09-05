#[derive(Debug)]
struct Calculate {
    a: i32,
    b: i32,
}

impl Calculate {
    fn add(&self) -> i32 {
        self.a + self.b
    }

    fn sub(&self) -> i32 {
        self.a - self.b
    }

    fn mul(&self) -> i32 {
        self.a * self.b
    }

    fn div(&self) -> i32 {
        if self.b == 0 {
            panic!("Cannot divide by zero");
        }

        self.a / self.b
    }
}

fn main() {
    let numbers = Calculate { a: 10, b: 10 };
    let methods = [numbers.add(),numbers.sub(), numbers.mul(), numbers.div()];
    let methods_string = ["add", "sub", "mul", "div"];

    for i in 0..=3 {
        println!("Metodo executado foi: {} e o resultado é: {}", methods_string[i], methods[i]);

    }

}
