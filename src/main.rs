mod cpu;
mod timer;
use cpu::Cpu;

fn main() {
    let mut cpu = Cpu::new();
    loop {
    	cpu.tick();
    }
}
