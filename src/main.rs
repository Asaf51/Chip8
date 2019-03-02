mod cpu;
mod timer;
mod display;
mod cartridge;
use cpu::Cpu;
use cartridge::Cartridge;
use std::{thread, time};

fn main() {
	let cartridge: Cartridge = Cartridge::new("/home/asaf/c8games/TICTAC");
    let mut cpu = Cpu::new(&cartridge);
    loop {
    	cpu.tick();
    	 thread::sleep(time::Duration::from_millis(2));
    }
}
