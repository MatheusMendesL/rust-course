#[derive(Debug)]
struct TreasureChest<T> {
    captain: String,
    treasure: T,
}

impl TreasureChest<String> {
    fn clean_treasure(&mut self) {
        self.treasure = self.treasure.trim().to_string()
    }
}

impl TreasureChest<[&str; 2]> {
    fn amount_of_treasure(&self) -> usize {
        self.treasure.len()
    }
}

impl<T> TreasureChest<T> {
    fn capital_capitan(&self) -> String {
        self.captain.to_uppercase()
    }
}

fn main() {
    let gold_chest = TreasureChest {
        captain: String::from("Captain"),
        treasure: "Gold"
    };
    println!("{}", gold_chest.capital_capitan());
    println!("{:?}", gold_chest);

    let mut silver_chest = TreasureChest {
        captain: String::from("Captain"),
        treasure: String::from("            silver      ")
    };
    silver_chest.clean_treasure();
    println!("{:?}", silver_chest);
    println!("{}", silver_chest.capital_capitan());

    let special_test = TreasureChest {
        captain: String::from("Captain"),
        treasure: [ "Gold", "Silver"]
    };

    let amount = special_test.amount_of_treasure();
    println!("{:?}", amount);
    println!("{}", special_test.capital_capitan());
    println!("{:?}", special_test);
}
