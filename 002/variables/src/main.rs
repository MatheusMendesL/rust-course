fn main() {
    let name: &str = "matheus";
    let kg: i32 = 90;
    if kg == 80 {
        println!("{} tá gordin", name)
    } else if kg > 80 {
        println!("{0} = boloris com peso de {1}", name, kg)
    }

    let _teste: i32 = 10;

    // colocando o numero no meio, pode reutilizar, ent no caso desse:
    // println!("{0} = boloris com peso de {1}, {0}", name, kg) o outro 0 retornaria meu nome novamente
}
