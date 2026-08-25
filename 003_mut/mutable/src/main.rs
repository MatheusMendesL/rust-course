fn main() {
    // sem esse mut a variavel n pode ser modificada
   let mut gym_reps: i32 = 20;
   println!("I plan to do {} reps", gym_reps);

   gym_reps = 10;
   println!("Now I plan to do {} reps", gym_reps);

   println!("{}", calc(5, 5));

   
}

fn calc(a:i32, b:i32) -> i32 {
    return a + b;
}