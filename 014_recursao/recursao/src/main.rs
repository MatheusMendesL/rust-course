fn countdown(seconds: i32){
    if seconds == 0{
        println!("Cabou");
    } else {
        println!("{seconds} seconds pra acabar");
        countdown(seconds - 1);
    }
}

fn main() {
   countdown(20);
}

// util, da pra usar como um loop mas usando recursão