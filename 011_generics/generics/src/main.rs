use std::ops::RangeInclusive;

#[allow(unused_variables)]
fn main() {
    let month_days: std::ops::Range<i8> = 1..31;

    for d in month_days {
        println!("{}", d);
    }

    let letters: std::ops::RangeInclusive<char> = 'a'..='z';

    // esse segundo é mais usado e dá no mesmo 🫡
    let letters: RangeInclusive<char> = 'a'..='z';

    for l in letters {
        println!("{}", l);
    }

}
