fn main() {
    let data = ("molly", 32, "marketing", 3.333314);
    dbg!(data);

    /* let name = data.0;
    let age = data.1;
    let course = data.2;
    let float_number = data.3; */

    let (name, age, course, float_number) = data;

    println!("Name {name}, age {age}, course: {course}, number float: {float_number:.2}");
}
