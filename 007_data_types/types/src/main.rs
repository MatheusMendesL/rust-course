fn main() {
    let days: usize = 55;
    let years: isize = -15_000;

    println!("{},   {}", days, years);

    let value: i32 = -15;
    // passar pra positivo
    println!("{}", value.abs());

    let empty_space = "                  teste";
    println!("{}", empty_space.trim());

    let number: i32 = 2;
    // exponenciação
    println!("{}", number.pow(10));

    let first_inicial: char = 'B';
    let emoji: char = '😁';

    println!("{}  {}", first_inicial.is_alphabetic(), emoji.is_alphabetic());
    println!("{}  {}", first_inicial.is_uppercase(), emoji.is_uppercase());
}
