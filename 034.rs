type Card = String;

#[derive(Debug)]
enum NormalUser {
    corrente,
    poupanca
}


#[derive(Debug)]
enum TypeUser {
    normal(NormalUser),
    rico
}

#[derive(Debug)]
struct User {
    type_user: TypeUser,
    username: String,
    pass: String,
}

impl User {
    fn new(type_user: TypeUser,username: String, pass: String) -> Self {
        Self {
            type_user,
            username,
            pass
        }
    }
}

#[derive(Debug)]
enum PaymentMethod {
    CreditCard(Card),
    DebitCard(Card),
    Pix(User)
}

fn main() {
    let user = User::new(TypeUser::normal((NormalUser::corrente)),String::from("matheus"), String::from("passwrd"));

    let payment = PaymentMethod::Pix(user);
    println!("{:#?}", payment);
}
