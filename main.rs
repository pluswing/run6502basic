mod cpu;
mod bus;
use self::cpu::CPU;
use self::bus::Bus;

fn main() {
    let cpu = CPU::new(Bus::new());
    cpu.reset();
    cpu.run_with_callback(move |cpu| {
      // TODO
    });
}
