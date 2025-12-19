mod cpu;
mod bus;
mod opscodes;
mod kernal;
use self::cpu::CPU;
use self::bus::Bus;
use std::fs::File;
use std::io::Read;


fn main() {

    // load basic rom
    let filepath = "bin/cbmbasic1.bin";
    let filepath = "bin/cbmbasic2.bin";
    let mut f = File::open(filepath).expect("no file found");
    let metadata = std::fs::metadata(filepath).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    println!("{} is {} byte!", filepath, metadata.len());
    f.read(&mut buffer).expect("buffer overflow");

    let mut cpu = CPU::new(Bus::new(&buffer));
    cpu.reset();
    cpu.run();
}
