mod cpu;

use cpu::CPU;

fn main() {

    println!("Hello, world! from Nick");

    let inst_cpu = CPU::new();

    println!("my CPU: {:#?}", inst_cpu);
}
