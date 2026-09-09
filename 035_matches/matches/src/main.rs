enum Os {
    Win,
    MacOS,
    Linux
}

fn main() {
    let pc = Os::Linux;
    let age = years_since_release(pc);
    println!("My pc is {age} years old");
}

fn years_since_release(os: Os) -> u32 {
    match os {
        Os::Win => 39,
        Os::MacOS => 23,
        Os::Linux => 34
    }
}