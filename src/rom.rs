use std::result;

pub struct Header {
    pub begin_code_execution_point: [u16; 2],
    pub game_title: [u8; 16],
    cartridge_type: CartridgeType,
    rom_size: u8, // number of banks
    ram_size: u8, // number of banks
    destination_code: DestinationCode,
    checksum: u16
}

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    RomTooShort,
    BadChecksum
}

// TODO: type-alias memory address to u16?
const HEADER_OFFSET: u32 = 0x0100;
const HEADER_LENGTH: u32 = 0x004F;

impl Header {
    pub fn parse(rom: &[u8]) -> Result<Header> {
        use std::slice::bytes::copy_memory;

        if (rom.len() as u32) < HEADER_OFFSET + HEADER_LENGTH {
            return Err(Error::RomTooShort)
        }
        //let header_bytes = rom[HEADER_OFFSET..HEADER_OFFSET + HEADER_LENGTH];
        let mut title = [0; 16];
        copy_memory(&rom[0x134..0x143], &mut title);
        let begin_execution = [(rom[100] as u16) << 8 | rom[101] as u16,
                               (rom[102] as u16) << 8 | rom[103] as u16];
        let cartridge_type = CartridgeType::from_u8(rom[0x147]);

        let header = Header {
            begin_code_execution_point: begin_execution,
            game_title: title,
            cartridge_type: cartridge_type,
            rom_size: 0, // TODO
            ram_size: 0, // TODO
            destination_code: DestinationCode::from_u8(rom[0x14A]),
            checksum: (rom[0x14E] as u16) << 8 | rom[0x14F] as u16
        };
        try!(header.check_checksum(rom));
        Ok(header)
    }

    fn check_checksum(&self, rom: &[u8]) -> Result<()> {
        use std::num::wrapping::OverflowingOps;

        let mut sum = 0u16;
        for byte in rom[..0x14E].iter().chain(rom[0x150..].iter()) {
            sum = sum.overflowing_add(*byte as u16).0;
        }
        if sum == self.checksum { Ok(()) } else { Err(Error::BadChecksum) }
    }
}

pub enum DestinationCode {
    Japanese,
    NonJapanese
}

impl DestinationCode {
    fn from_u8(code: u8) -> DestinationCode {
        match code {
            0 => DestinationCode::Japanese,
            _ => DestinationCode::NonJapanese
        }
    }
}

pub struct CartridgeType {
    memory_bank: Option<MemoryBankController>
}

impl CartridgeType {
    fn from_u8(n: u8) -> CartridgeType {
        CartridgeType { memory_bank: MemoryBankController::from_u8(n) }
    }
}

pub enum MemoryBankController {
    Type1,
    Type2,
    Type3,
    Type5,
}

impl MemoryBankController {
    pub fn from_u8(n: u8) -> Option<MemoryBankController> {
        match n {
            1 | 2 | 3 => Some(MemoryBankController::Type1),
            5 | 6 => Some(MemoryBankController::Type2),
            0x12 | 0x13 => Some(MemoryBankController::Type3),
            0x19...0x1E => Some(MemoryBankController::Type5),
            _ => None
        }
    }
}
