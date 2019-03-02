mod cpu;
mod timer;
mod display;
mod cartridge;
use cpu::Cpu;
use cartridge::Cartridge;

fn main() {
	let cartridge: Cartridge = Cartridge::new("/home/asaf/c8games/BLINKY");
    let mut cpu = Cpu::new(&cartridge);
    loop {
    	cpu.tick();
    }
}
