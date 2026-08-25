#[allow(unused_variables)]
fn main() {
    let month_days = 1..31;

    for i in month_days  {
        dbg!(i);
    }
    
    let month_days = 1..=31;

    for i in month_days  {
        dbg!(i);
    }

    let alphabet = 'a'..='z';

    for l in alphabet {
        println!("{}", l);
    }
}
