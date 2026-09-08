type NumberCard = String;
type Mail = String;
type Pass = String;
#[derive(Debug)]
enum PaymentMethodType {
    Credit(NumberCard),
    Debit(NumberCard),
    Pix(Mail, Pass),
}

fn main() {
    let visa = PaymentMethodType::Credit(String::from("0034-5678-9012-34567"));
    let mastercard = PaymentMethodType::Debit(String::from("0034-5678-9012-34597"));
    let pix = PaymentMethodType::Pix(String::from("matheus@gmail.com"), String::from("password"));
}
