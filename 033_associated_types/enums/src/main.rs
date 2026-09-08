#[derive(Debug)]
enum PaymentMethodType {
    Credit(String),
    Debit(String),
    Pix(String),
}

fn main() {
    let visa = PaymentMethodType::Credit(String::from("0034-5678-9012-34567"));
    let mastercard = PaymentMethodType::Debit(String::from("0034-5678-9012-34597"))
}


