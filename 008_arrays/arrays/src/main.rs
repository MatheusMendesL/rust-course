fn main() {
    let numbers: [i32; 5] = [1,2,3,4,5];

    println!("{}", numbers[0]);

    let chars = ['a', 'b', 'c'];
    println!("{}", chars.len());

    // pra formatar e pro display ser certo
    println!("{:#?}", numbers);

    dbg!(chars);
}
 