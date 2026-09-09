fn main() {

    let musical_instruments = [
        String::from("Guitar"),
        String::from("Drums"),
        String::from("Bass"),
    ];

    let bass = musical_instruments.get(2);
    play(bass);

    let invalid = musical_instruments.get(10);
    play(invalid);
}

fn play(instrument: Option<&String>) {
    match instrument {
        Some(instrument) => {
            println!("{:?}", instrument);
        }
        None => {
            println!("None");
        }
    }
}
