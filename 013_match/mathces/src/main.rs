fn main() {
    let evaluation: bool = true;
    
    match evaluation  {
        true => {
            println!("True");
        }

        false => {
            println!("False");
        }
    }

    let season = "Winter";

    match season {
        "Spring" => {
            println!("Spring");
        }

        "Fall" => {
            println!("Fall");
        }

        "Summer" => {
            println!("Summer");
        }

        "Winter" => {
            println!("Winter");
        }

        // é um default
        _ => {
            println!("Nothing");
        }
    }

    let number = 13787943;
    match number {
        value if value % 2 == 0 => println!("par"),
        value if value % 2 == 1 => println!("impar"),
        _ => unreachable!(),
    }
    
}
