// hours, minutes
struct ShortDuration(
    u32,
    u32
);

// months, years
struct LongDuration(
    u32,
    u32,
);

fn main() {
    let work_shift = ShortDuration(5, 35);
    println!("{} hours {} minutes", work_shift.0, work_shift.1);

    let era = LongDuration(5, 3);
    println!("{} years {} months", era.0, era.1);

    go_to_work(work_shift);

    /*let work_shift= (8,0);
    let era = (5,3);

    go_to_work(work_shift);
    go_to_work(era);*/
}

fn go_to_work(length: ShortDuration){
    println!("Passing time {} hours {} minutes", length.0, length.1);
}