fn main() {
    let ok: Result<i32, &str> = Ok(5);
    println!("{ok:?}");
    let err: Result<i32, &str> = Err("erro");
    println!("{err:?}");
}
