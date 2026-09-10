use std::mem;

#[derive(Debug, Copy, Clone)]
enum MyOpt<T> {
    Some(T),
    None
}

impl<T> MyOpt<T> {
    fn unwrap(self) -> T {
        match self {
            MyOpt::Some(value) => {
                value
            }

            MyOpt::None => {
                panic!("Value does not exist");
            }
        }
    }

    fn unwrap_or(self, fallback: T) -> T {
        match self {
            MyOpt::Some(value) => {
                value
            }

            MyOpt::None => {
                fallback
            }
        }
    }

    fn is_some(&self) -> bool {
        match self {
            MyOpt::Some(_) => {
                true
            }
            MyOpt::None => {
                false
            }
        }
    }

    fn is_none(&self) -> bool {
        match self {
            MyOpt::Some(_) => {
                false
            }
            MyOpt::None => {
                true
            }
        }
    }

    fn take(&mut self) -> (MyOpt<T>, T) {

        let old_self = mem::replace(self, MyOpt::None);
        match old_self {
            MyOpt::Some(value) => {
                (MyOpt::None, value)
            }

            MyOpt::None => {
                panic!("Impossible to use None on this method")
            }
        }
    }

    fn expect(self, msg: &str) -> T {
        match self {
            MyOpt::Some(value) => {
                value
            }

            MyOpt::None => {
                panic!("{}", msg);
            }
        }
    }
}


fn main() {
    let some = MyOpt::Some("teste");
    println!("{}", some.unwrap());


    let some = MyOpt::Some(String::from("teste"));
    println!("{}", some.unwrap());

    let none = MyOpt::None;
    print!("{}", none.unwrap_or("fallback"));
}
