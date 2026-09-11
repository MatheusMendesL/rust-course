use std::mem;

#[derive(Debug, Copy, Clone)]
enum MyOpt<T> {
    Some(T),
    None
}

#[derive(Debug, Clone, Copy)]
enum MyResult<T, E> {
    Ok(T),
    Err(E)
}

#[allow(dead_code)]
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

    fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T,
    {
        match self {
            MyOpt::Some(value) => value,
            MyOpt::None => f(),
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

    fn take(&mut self) -> MyOpt<T> {
        mem::replace(self, MyOpt::None)
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
    fn replace(&mut self, value: T) -> MyOpt<T> {
        mem::replace(self, MyOpt::Some(value))
    }

    fn as_ref(&self) -> MyOpt<&T> {
        match self {
            MyOpt::Some(value) => {
                MyOpt::Some(&value)
            }
            MyOpt::None => {
                MyOpt::None
            }
        }
    }

    fn as_mut(&mut self) -> MyOpt<&mut T> {
        match self {
            MyOpt::Some(value) => {
                MyOpt::Some(value)
            }
            MyOpt::None => {
                MyOpt::None
            }
        }
    }
}



#[allow(dead_code)]
impl<T, E> MyResult<T, E> {
    fn unwrap(self) -> T {
        match self {
            MyResult::Ok(value) => value,
            MyResult::Err(_) => {
                panic!("Result contains an error");
            }
        }
    }

    fn unwrap_or(self, fallback: T) -> T {
        match self {
            MyResult::Ok(value) => value,
            MyResult::Err(_) => fallback,
        }
    }

    fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T,
    {
        match self {
            MyResult::Ok(value) => value,
            MyResult::Err(_) => f(),
        }
    }

    fn expect(self, msg: &str) -> T {
        match self {
            MyResult::Ok(value) => value,
            MyResult::Err(_) => {
                panic!("{}", msg);
            }
        }
    }

    fn unwrap_err(self) -> E {
        match self {
            MyResult::Ok(_) => {
                panic!("Result contains a value");
            }
            MyResult::Err(error) => error,
        }
    }

    fn expect_err(self, msg: &str) -> E {
        match self {
            MyResult::Ok(_) => {
                panic!("{}", msg);
            }
            MyResult::Err(error) => error,
        }
    }

    fn is_ok(&self) -> bool {
        match self {
            MyResult::Ok(_) => true,
            MyResult::Err(_) => false,
        }
    }

    fn is_err(&self) -> bool {
        match self {
            MyResult::Ok(_) => false,
            MyResult::Err(_) => true,
        }
    }

    fn ok(self) -> MyOpt<T> {
        match self {
            MyResult::Ok(value) => MyOpt::Some(value),
            MyResult::Err(_) => MyOpt::None,
        }
    }

    fn err(self) -> MyOpt<E> {
        match self {
            MyResult::Ok(_) => MyOpt::None,
            MyResult::Err(error) => MyOpt::Some(error),
        }
    }

    fn replace_ok(&mut self, value: T) -> MyResult<T, E> {
        mem::replace(self, MyResult::Ok(value))
    }

    fn replace_err(&mut self, error: E) -> MyResult<T, E> {
        mem::replace(self, MyResult::Err(error))
    }

    fn as_ref(&self) -> MyResult<&T, &E> {
        match self {
            MyResult::Ok(value) => MyResult::Ok(value),
            MyResult::Err(error) => MyResult::Err(error),
        }
    }

    fn as_mut(&mut self) -> MyResult<&mut T, &mut E> {
        match self {
            MyResult::Ok(value) => MyResult::Ok(value),
            MyResult::Err(error) => MyResult::Err(error),
        }
    }
}


struct MyLib<T, E> {
    my_opt: MyOpt<T>,
    my_result: MyResult<T, E>
}

#[allow(dead_code)]
impl<T, E> MyLib<T, E> {
    fn unwrap(MyLib { my_opt, my_result }: Self) -> (T, T) {
        (my_opt.unwrap(), my_result.unwrap())
    }

    fn unwrap_or(MyLib { my_opt, my_result }: Self, fallback_opt: T, fallback_res: T) -> (T, T) {
        (my_opt.unwrap_or(fallback_opt), my_result.unwrap_or(fallback_res))
    }

    fn expect(MyLib { my_opt, my_result }: Self, msg_opt: &str, msg_res: &str) -> (T, T) {
        (my_opt.expect(msg_opt), my_result.expect(msg_res))
    }

    fn is_both_ok(MyLib { my_opt, my_result }: &Self) -> bool {
        if my_opt.is_some() && my_result.is_ok() {
            true
        } else {
            false
        }
    }

    fn is_one_ok(MyLib { my_opt, my_result }: &Self) -> bool {
        if my_opt.is_some() || my_result.is_ok() {
            true
        } else {
            false
        }
    }

    fn is_both_none(MyLib { my_opt, my_result }: &Self) -> bool {
        if my_opt.is_none() && my_result.is_err() {
            true
        } else {
            false
        }
    }
}

fn main() {
    println!("Hello, world!");
}
