pub struct Bus {
    cpu_ram: [u8; 1024 * 4]
}

impl Bus {
    pub fn new(apu: NesAPU) -> Self {
        Self {
            cpu_ram: [0; 1024 * 4],
        }
    }
}

pub trait Mem {
    fn mem_read(&mut self, addr: u16) -> u8;
    fn mem_write(&mut self, addr: u16, data: u8);
}

impl Mem for Bus {
    fn mem_read(&mut self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x0FFF => {
                // RAM
                self.cpu_ram[addr]
            }
            _ => {
                warn!("Ignoreing mem access at {:X}", addr);
                0
            }
        }
    }

    fn mem_write(&mut self, addr: u16, data: u8) {
        match addr {
            0x0000..=0x0FFF => {
                // RAM
                self.cpu_ram[addr] = data;
            }
            _ => {
                warn!("Ignoreing mem write access at {:X}", addr, value);
                0
            }
        }
    }
}
