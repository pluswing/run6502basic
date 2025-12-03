use log::{warn};

pub struct Bus {
    cpu_ram: [u8; 1024 * 4],
    basic_rom: Vec<u8>,
}

impl Bus {
    pub fn new(basic_rom: &Vec<u8>) -> Self {
        Self {
            cpu_ram: [0; 1024 * 4],
            basic_rom: basic_rom.to_vec(),
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
                self.cpu_ram[addr as usize]
            }
            0xC000..=0xDFFF => {
              self.basic_rom[(addr - 0xC000) as usize]
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
                self.cpu_ram[addr as usize] = data;
            }
            _ => {
                warn!("Ignoreing mem write access at {:X} => {:X}", addr, data);
            }
        }
    }
}
