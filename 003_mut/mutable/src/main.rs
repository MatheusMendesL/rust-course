fn main() {
    // sem esse mut a variavel n pode ser modificada
   let mut gym_reps: i32 = 20;
   println!("I plan to do {} reps", gym_reps);

   gym_reps = 10;
   println!("Now I plan to do {} reps", gym_reps)
}
