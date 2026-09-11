use std::mem;

#[derive(Debug, Clone, Copy)]
enum MyResult<T> {
    Ok(T),
    Err(T)
}

#[allow(dead_code)]
impl<T> MyResult<T> {
    fn unwrap(self) -> T {
        match self {
            MyResult::Ok(value) => {
                value
            }
            MyResult::Err(value) => {
                value
            }
        }
    }

    fn unwrap_or(self, fallback: T) -> T {
        match self {
            MyResult::Ok(value) => {
                value
            }
            MyResult::Err(_) => {
                fallback
            }
        }
    }

    fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T,
    {
        match self {
            MyResult::Ok(value) => {
                value
            }

            MyResult::Err(_) => {
                f()
            }
        }
    }

    fn expect(self, msg: &str) -> T {
        match self {
            MyResult::Ok(value) => {
                value
            }

            MyResult::Err(_) => {
                panic!("Erro: {}", msg)
            }
        }
    }

    fn is_ok(&self) -> bool {
        match self {
            MyResult::Ok(_) => {
                true
            }
            MyResult::Err(_) => {
                false
            }
        }
    }

    fn is_err(&self) -> bool {
        match self {
            MyResult::Ok(_) => {
                false
            }
            MyResult::Err(_) => {
                true
            }
        }
    }

    fn ok(self) -> Option<T> {
        match self {
            MyResult::Ok(value) => Some(value),
            MyResult::Err(_) => None,
        }
    }

    fn err(self) -> Option<T> {
        match self {
            MyResult::Ok(_) => None,
            MyResult::Err(value) => Some(value),
        }
    }

    fn replace_ok(&mut self, value: T) -> MyResult<T> {
        mem::replace(self, MyResult::Ok(value))
    }

    fn replace_err(&mut self, value: T) -> MyResult<T> {
        mem::replace(self, MyResult::Err(value))
    }

    fn as_ref(&self) -> MyResult<&T>{
        match self {
            MyResult::Ok(value) => {
                MyResult::Ok(&value)
            }

            MyResult::Err(value) => {
                MyResult::Err(&value)
            }
        }
    }

    fn as_mut(&mut self) -> MyResult<&mut T> {
        match self {
            MyResult::Ok(value) => {
                MyResult::Ok(value)
            }

            MyResult::Err(value) => {
                MyResult::Err(value)
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
}
