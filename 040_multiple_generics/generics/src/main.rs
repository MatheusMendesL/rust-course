fn make_tuple<T, U>(first: T, second: U) -> (T, U) {
    (first, second)
}

fn main() {
    make_tuple("a", 5);
}
