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
            0x0000..=0x7FFF => {
                // RAM
                let mirror_addr = addr % 0x1000;
                self.cpu_ram[mirror_addr as usize]
            }
            // cbmbasic1に合わせて、お尻を調整。
            0xC000..=0xE1E1 => {
              // 0xC000..=0xDFFF = 8192 byte => 8KB
              // actually 8673 byte
              // 49152(0xC000) + 8673 = 57825 = 0xE1E1
              self.basic_rom[(addr - 0xC000) as usize]
            }

            0xFFE7 => {
              // CLALL
              0x60 // RTS
            }
            0xFFCF => {
              // CHRIN
              0x60 // RTS
            }

            // // BREAK
            // 0xFF00 => {
            //   0x60 // RTS
            // }
            // 0xFFFE => {
            //   // BREAK TODO PET用に変える必要があるかも。
            //   0x00
            // }
            // 0xFFFF => {
            //   0xFF
            // }
            _ => {
                todo!("Ignoreing mem read access at {:X}", addr);
                0
            }
        }
    }

    fn mem_write(&mut self, addr: u16, data: u8) {
        match addr {
            0x0000..=0x7FFF => {
                // RAM
                let mirror_addr = addr % 0x1000;
                self.cpu_ram[mirror_addr as usize] = data;
            }
            0xFFD2 => {
              // CHROUT, MONCOUT
              // -> 画面出力系
            }
            _ => {
                todo!("Ignoreing mem write access at {:X} => {:X}", addr, data);
            }
        }
    }
}
