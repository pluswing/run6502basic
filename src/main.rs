mod cpu;
mod bus;
mod opscodes;
use self::cpu::CPU;
use self::bus::Bus;

fn main() {
    let mut cpu = CPU::new(Bus::new());
    cpu.reset();
    cpu.run();
}
