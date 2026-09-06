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
}
