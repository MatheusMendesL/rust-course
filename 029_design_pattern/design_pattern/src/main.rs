#[derive(Debug)]
struct Computer {
    cpu: String,
    memory: u32,
    hard_drive_capacity: u32,
}

impl Computer {
    fn new(cpu: String, memory: u32, hard_drive_capacity: u32) -> Self {
        Self {
            cpu,
            memory,
            hard_drive_capacity,
        }
    }

    fn upgrade_cpu(&mut self, new_cpu: String) -> &mut Self {
        self.cpu = new_cpu;
        self
    }

    fn upgrade_memory(&mut self, new_memory: u32) -> &mut Self {
        self.memory = new_memory;
        self
    }

    fn upgrade_hard_drive(&mut self, new_hard_drive: u32) -> &mut Self {
        self.hard_drive_capacity = new_hard_drive;
        self
    }
}

fn main() {
    let mut computer = Computer::new(String::from("Ryzen 5 8600G"), 32, 512);
    println!("{:?}", computer);
    
    computer
        .upgrade_cpu(String::from("Ryzen 7 7800x"))
        .upgrade_memory(64)
        .upgrade_hard_drive(1012);

    println!("{:?}", computer);
}
