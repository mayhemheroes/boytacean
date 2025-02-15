use core::fmt;
use std::{
    cmp::max,
    fmt::{Display, Formatter},
};

use crate::{debugln, gb::GameBoyMode, util::read_file, warnln};

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

pub const ROM_BANK_SIZE: usize = 16384;
pub const RAM_BANK_SIZE: usize = 8192;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub enum RomType {
    RomOnly = 0x00,
    Mbc1 = 0x01,
    Mbc1Ram = 0x02,
    Mbc1RamBattery = 0x03,
    Mbc2 = 0x05,
    Mbc2Battery = 0x06,
    RomRam = 0x08,
    RomRamBattery = 0x09,
    Mmm01 = 0x0b,
    Mmm01Ram = 0x0c,
    Mmm01RamBattery = 0x0d,
    Mbc3TimerBattery = 0x0f,
    Mbc3TimerRamBattery = 0x10,
    Mbc3 = 0x11,
    Mbc3Ram = 0x12,
    Mbc3RamBattery = 0x13,
    Mbc5 = 0x19,
    Mbc5Ram = 0x1a,
    Mbc5RamBattery = 0x1b,
    Mbc5Rumble = 0x1c,
    Mbc5RumbleRam = 0x1d,
    Mbc5RumbleRamBattery = 0x1e,
    Mbc6 = 0x20,
    Mbc7SensorRumbleRamBattery = 0x22,
    PocketCamera = 0xfc,
    BandaiTama5 = 0xfd,
    HuC3 = 0xfe,
    HuC1RamBattery = 0xff,
    Unknown = 0xef,
}

impl RomType {
    pub fn description(&self) -> &'static str {
        match self {
            RomType::RomOnly => "ROM Only",
            RomType::Mbc1 => "MBC1",
            RomType::Mbc1Ram => "MBC1 + RAM",
            RomType::Mbc1RamBattery => "MBC1 + RAM + Battery",
            RomType::Mbc2 => "MBC2",
            RomType::Mbc2Battery => "MBC2 + RAM",
            RomType::RomRam => "ROM + RAM",
            RomType::RomRamBattery => "ROM + RAM + BATTERY",
            RomType::Mmm01 => "MMM01",
            RomType::Mmm01Ram => "MMM01 + RAM",
            RomType::Mmm01RamBattery => "MMM01 + RAM + BATTERY",
            RomType::Mbc3TimerBattery => "MBC3 + TIMER + BATTERY",
            RomType::Mbc3TimerRamBattery => "MBC3 + TIMER + RAM + BATTERY",
            RomType::Mbc3 => "MBC3",
            RomType::Mbc3Ram => "MBC3 + RAM",
            RomType::Mbc3RamBattery => "MBC3 + RAM + BATTERY",
            RomType::Mbc5 => "MBC5",
            RomType::Mbc5Ram => "MBC5 + RAM",
            RomType::Mbc5RamBattery => "MBC5 + RAM + BATTERY",
            RomType::Mbc5Rumble => "MBC5 + RUMBLE",
            RomType::Mbc5RumbleRam => "MBC5 + RUMBLE + RAM",
            RomType::Mbc5RumbleRamBattery => "MBC5 + RUMBLE + RAM + BATTERY",
            RomType::Mbc6 => "MBC6",
            RomType::Mbc7SensorRumbleRamBattery => "MBC6 + SENSOR + RUMBLE + RAM + BATTERY",
            RomType::PocketCamera => "POCKET CAMERA",
            RomType::BandaiTama5 => "BANDAI TAMA5",
            RomType::HuC3 => "HuC3",
            RomType::HuC1RamBattery => "HuC1 + RAM + BATTERY",
            RomType::Unknown => "Unknown",
        }
    }
}

impl Display for RomType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub enum RomSize {
    Size32K,
    Size64K,
    Size128K,
    Size256K,
    Size512K,
    Size1M,
    Size2M,
    Size4M,
    Size8M,
    SizeUnknown,
}

impl RomSize {
    pub fn description(&self) -> &'static str {
        match self {
            RomSize::Size32K => "32 KB",
            RomSize::Size64K => "64 KB",
            RomSize::Size128K => "128 KB",
            RomSize::Size256K => "256 KB",
            RomSize::Size512K => "512 KB",
            RomSize::Size1M => "1 MB",
            RomSize::Size2M => "2 MB",
            RomSize::Size4M => "4 MB",
            RomSize::Size8M => "8 MB",
            RomSize::SizeUnknown => "Unknown",
        }
    }

    pub fn rom_banks(&self) -> u16 {
        match self {
            RomSize::Size32K => 2,
            RomSize::Size64K => 4,
            RomSize::Size128K => 8,
            RomSize::Size256K => 16,
            RomSize::Size512K => 32,
            RomSize::Size1M => 64,
            RomSize::Size2M => 128,
            RomSize::Size4M => 256,
            RomSize::Size8M => 512,
            RomSize::SizeUnknown => 0,
        }
    }
}

impl Display for RomSize {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub enum RamSize {
    NoRam,
    Unused,
    Size8K,
    Size16K,
    Size32K,
    Size64K,
    Size128K,
    SizeUnknown,
}

impl RamSize {
    pub fn description(&self) -> &'static str {
        match self {
            RamSize::NoRam => "No RAM",
            RamSize::Unused => "Unused",
            RamSize::Size8K => "8 KB",
            RamSize::Size16K => "16 KB",
            RamSize::Size32K => "32 KB",
            RamSize::Size128K => "128 KB",
            RamSize::Size64K => "64 KB",
            RamSize::SizeUnknown => "Unknown",
        }
    }

    pub fn ram_banks(&self) -> u16 {
        match self {
            RamSize::NoRam => 0,
            RamSize::Unused => 0,
            RamSize::Size8K => 1,
            RamSize::Size16K => 2,
            RamSize::Size32K => 4,
            RamSize::Size64K => 8,
            RamSize::Size128K => 16,
            RamSize::SizeUnknown => 0,
        }
    }
}

impl Display for RamSize {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub enum CgbMode {
    NoCgb = 0x00,
    CgbCompatible = 0x80,
    CgbOnly = 0xc0,
}

impl CgbMode {
    pub fn description(&self) -> &'static str {
        match self {
            CgbMode::NoCgb => "No CGB support",
            CgbMode::CgbCompatible => "CGB backwards compatible",
            CgbMode::CgbOnly => "CGB only",
        }
    }
}

impl Display for CgbMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

/// Structure that defines the ROM and ROM contents
/// of a Game Boy cartridge. Should correctly address
/// the specifics of all the major MBCs (Memory Bank
/// Controllers).
#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[derive(Clone)]
pub struct Cartridge {
    /// The complete data of the ROM cartridge, should
    /// include the complete set o ROM banks.
    rom_data: Vec<u8>,

    /// The base RAM that is going to be used to store
    /// temporary data for basic cartridges.
    ram_data: Vec<u8>,

    /// The MBC (Memory Bank Controller) to be used for
    /// RAM and ROM access on the current cartridge.
    mbc: &'static Mbc,

    /// The number of ROM banks (of 8KB) that are available
    /// to the current cartridge, this is a computed value
    /// to allow improved performance.
    rom_bank_count: u16,

    /// The number of RAM banks (of 8KB) that are available
    /// to the current cartridge, this is a computed value
    /// to allow improved performance.
    ram_bank_count: u16,

    /// The offset address to the ROM bank (#1) that is
    /// currently in use by the ROM cartridge.
    rom_offset: usize,

    /// The offset address to the ERAM bank that is
    /// currently in use by the ROM cartridge.
    ram_offset: usize,

    /// If the RAM access ia enabled, this flag allows
    /// control of memory access to avoid corruption.
    ram_enabled: bool,

    // The final offset of the last character of the title
    // that is considered to be non zero (0x0) so that a
    // proper safe conversion to UTF-8 string can be done.
    title_offset: usize,
}

impl Cartridge {
    pub fn new() -> Self {
        Self {
            rom_data: vec![],
            ram_data: vec![],
            mbc: &NO_MBC,
            rom_bank_count: 0,
            ram_bank_count: 0,
            rom_offset: 0x4000,
            ram_offset: 0x0000,
            ram_enabled: false,
            title_offset: 0x0143,
        }
    }

    pub fn from_data(data: &[u8]) -> Self {
        let mut cartridge = Cartridge::new();
        cartridge.set_data(data);
        cartridge
    }

    pub fn from_file(path: &str) -> Self {
        let data = read_file(path);
        Self::from_data(&data)
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr & 0xf000 {
            0x0000 | 0x1000 | 0x2000 | 0x3000 | 0x4000 | 0x5000 | 0x6000 | 0x7000 => {
                (self.mbc.read_rom)(self, addr)
            }
            0xa000 | 0xb000 => (self.mbc.read_ram)(self, addr),
            _ => {
                debugln!("Reading from unknown Cartridge control 0x{:04x}", addr);
                0x00
            }
        }
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        match addr & 0xf000 {
            0x0000 | 0x1000 | 0x2000 | 0x3000 | 0x4000 | 0x5000 | 0x6000 | 0x7000 => {
                (self.mbc.write_rom)(self, addr, value)
            }
            0xa000 | 0xb000 => (self.mbc.write_ram)(self, addr, value),
            _ => debugln!("Writing to unknown Cartridge address 0x{:04x}", addr),
        }
    }

    pub fn data(&self) -> &Vec<u8> {
        &self.rom_data
    }

    pub fn get_bank(&self, index: u8) -> &[u8] {
        let start = index as usize * ROM_BANK_SIZE;
        let end = (index + 1) as usize * ROM_BANK_SIZE;
        &self.rom_data[start..end]
    }

    pub fn get_mbc(&self) -> &'static Mbc {
        match self.rom_type() {
            RomType::RomOnly => &NO_MBC,
            RomType::Mbc1 => &MBC1,
            RomType::Mbc1Ram => &MBC1,
            RomType::Mbc1RamBattery => &MBC1,
            RomType::Mbc3TimerBattery => &MBC3,
            RomType::Mbc3TimerRamBattery => &MBC3,
            RomType::Mbc3 => &MBC3,
            RomType::Mbc3Ram => &MBC3,
            RomType::Mbc3RamBattery => &MBC3,
            RomType::Mbc5 => &MBC5,
            RomType::Mbc5Ram => &MBC5,
            RomType::Mbc5RamBattery => &MBC5,
            RomType::Mbc5Rumble => &MBC5,
            RomType::Mbc5RumbleRam => &MBC5,
            RomType::Mbc5RumbleRamBattery => &MBC5,
            rom_type => panic!("No MBC controller available for {}", rom_type),
        }
    }

    pub fn set_rom_bank(&mut self, rom_bank: u8) {
        self.rom_offset = rom_bank as usize * ROM_BANK_SIZE;
    }

    pub fn set_ram_bank(&mut self, ram_bank: u8) {
        self.ram_offset = ram_bank as usize * RAM_BANK_SIZE;
    }

    fn set_data(&mut self, data: &[u8]) {
        self.rom_data = data.to_vec();
        self.rom_offset = 0x4000;
        self.ram_offset = 0x0000;
        self.set_mbc();
        self.set_computed();
        self.set_title_offset();
        self.allocate_ram();
        self.set_rom_bank(1);
        self.set_ram_bank(0);
    }

    fn set_mbc(&mut self) {
        self.mbc = self.get_mbc();
    }

    fn set_computed(&mut self) {
        self.rom_bank_count = self.rom_size().rom_banks();
        self.ram_bank_count = self.ram_size().ram_banks();
    }

    pub fn set_title_offset(&mut self) {
        let mut offset: usize = 0;
        for byte in &self.rom_data[0x0134..=0x0143] {
            if *byte == 0u8 {
                break;
            }

            // in we're at the final byte of the title and the value
            // is one that is reserved for CGB compatibility testing
            // then we must ignore it for title processing purposes
            if offset > 14
                && (*byte == CgbMode::CgbCompatible as u8 || *byte == CgbMode::CgbOnly as u8)
            {
                break;
            }

            offset += 1;
        }
        self.title_offset = 0x0134 + offset;
    }

    fn allocate_ram(&mut self) {
        let ram_banks = max(self.ram_size().ram_banks(), 1);
        self.ram_data = vec![0u8; ram_banks as usize * RAM_BANK_SIZE];
    }
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
impl Cartridge {
    pub fn title(&self) -> String {
        String::from(
            std::str::from_utf8(&self.rom_data[0x0134..self.title_offset])
                .unwrap()
                .trim(),
        )
    }

    pub fn cgb_flag(&self) -> CgbMode {
        match self.rom_data[0x0143] {
            0x80 => CgbMode::CgbCompatible,
            0xc0 => CgbMode::CgbOnly,
            _ => CgbMode::NoCgb,
        }
    }

    pub fn gb_mode(&self) -> GameBoyMode {
        match self.cgb_flag() {
            CgbMode::CgbCompatible | CgbMode::CgbOnly => GameBoyMode::Cgb,
            _ => GameBoyMode::Dmg,
        }
    }

    /// A cartridge is considered legacy if it does
    /// not have a CGB flag bit (bit 7 of 0x0143) set.
    /// These are the monochromatic only Cartridges built
    /// for the original DMG Game Boy.
    pub fn is_legacy(&self) -> bool {
        self.rom_data[0x0143] & 0x80 == 0x00
    }

    pub fn rom_type(&self) -> RomType {
        match self.rom_data[0x0147] {
            0x00 => RomType::RomOnly,
            0x01 => RomType::Mbc1,
            0x02 => RomType::Mbc1Ram,
            0x03 => RomType::Mbc1RamBattery,
            0x05 => RomType::Mbc2,
            0x06 => RomType::Mbc2Battery,
            0x08 => RomType::RomRam,
            0x09 => RomType::RomRamBattery,
            0x0b => RomType::Mmm01,
            0x0c => RomType::Mmm01Ram,
            0x0d => RomType::Mmm01RamBattery,
            0x0f => RomType::Mbc3TimerBattery,
            0x10 => RomType::Mbc3TimerRamBattery,
            0x11 => RomType::Mbc3,
            0x12 => RomType::Mbc3Ram,
            0x13 => RomType::Mbc3RamBattery,
            0x19 => RomType::Mbc5,
            0x1a => RomType::Mbc5Ram,
            0x1b => RomType::Mbc5RamBattery,
            0x1c => RomType::Mbc5Rumble,
            0x1d => RomType::Mbc5RumbleRam,
            0x1e => RomType::Mbc5RumbleRamBattery,
            0x20 => RomType::Mbc6,
            0x22 => RomType::Mbc7SensorRumbleRamBattery,
            0xfc => RomType::PocketCamera,
            0xfd => RomType::BandaiTama5,
            0xfe => RomType::HuC3,
            0xff => RomType::HuC1RamBattery,
            _ => RomType::Unknown,
        }
    }

    pub fn rom_size(&self) -> RomSize {
        match self.rom_data[0x0148] {
            0x00 => RomSize::Size32K,
            0x01 => RomSize::Size64K,
            0x02 => RomSize::Size128K,
            0x03 => RomSize::Size256K,
            0x04 => RomSize::Size512K,
            0x05 => RomSize::Size1M,
            0x06 => RomSize::Size2M,
            0x07 => RomSize::Size4M,
            0x08 => RomSize::Size8M,
            _ => RomSize::SizeUnknown,
        }
    }

    pub fn ram_size(&self) -> RamSize {
        match self.rom_data[0x0149] {
            0x00 => RamSize::NoRam,
            0x01 => RamSize::Unused,
            0x02 => RamSize::Size8K,
            0x03 => RamSize::Size32K,
            0x04 => RamSize::Size128K,
            0x05 => RamSize::Size64K,
            _ => RamSize::SizeUnknown,
        }
    }

    pub fn rom_type_s(&self) -> String {
        String::from(self.rom_type().description())
    }

    pub fn rom_size_s(&self) -> String {
        String::from(self.rom_size().description())
    }

    pub fn ram_size_s(&self) -> String {
        String::from(self.ram_size().description())
    }

    pub fn has_battery(&self) -> bool {
        matches!(
            self.rom_type(),
            RomType::Mbc1RamBattery
                | RomType::Mbc2Battery
                | RomType::RomRamBattery
                | RomType::Mmm01RamBattery
                | RomType::Mbc3TimerBattery
                | RomType::Mbc3TimerRamBattery
                | RomType::Mbc3RamBattery
                | RomType::Mbc5RamBattery
                | RomType::Mbc5RumbleRamBattery
                | RomType::Mbc7SensorRumbleRamBattery
                | RomType::HuC1RamBattery
        )
    }

    pub fn ram_data_eager(&self) -> Vec<u8> {
        self.ram_data.clone()
    }

    pub fn set_ram_data(&mut self, ram_data: Vec<u8>) {
        self.ram_data = ram_data;
    }

    pub fn description(&self, column_length: usize) -> String {
        let name_l = format!("{:width$}", "Name", width = column_length);
        let type_l = format!("{:width$}", "Type", width = column_length);
        let rom_size_l = format!("{:width$}", "ROM Size", width = column_length);
        let ram_size_l = format!("{:width$}", "RAM Size", width = column_length);
        let cgb_l = format!("{:width$}", "CGB Mode", width = column_length);
        format!(
            "{}  {}\n{}  {}\n{}  {}\n{}  {}\n{}  {}",
            name_l,
            self.title(),
            type_l,
            self.rom_type(),
            rom_size_l,
            self.rom_size(),
            ram_size_l,
            self.ram_size(),
            cgb_l,
            self.cgb_flag()
        )
    }
}

impl Default for Cartridge {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for Cartridge {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description(9))
    }
}

pub struct Mbc {
    pub name: &'static str,
    pub read_rom: fn(rom: &Cartridge, addr: u16) -> u8,
    pub write_rom: fn(rom: &mut Cartridge, addr: u16, value: u8),
    pub read_ram: fn(rom: &Cartridge, addr: u16) -> u8,
    pub write_ram: fn(rom: &mut Cartridge, addr: u16, value: u8),
}

pub static NO_MBC: Mbc = Mbc {
    name: "No MBC",
    read_rom: |rom: &Cartridge, addr: u16| -> u8 { rom.rom_data[addr as usize] },
    write_rom: |_rom: &mut Cartridge, addr: u16, _value: u8| {
        match addr {
            // ignores this address as Tetris and some other games write
            // to this address for some reason (probably related to
            // some kind of MBC1 compatibility issue)
            0x2000 => (),
            _ => panic!("Writing to unknown Cartridge ROM location 0x{:04x}", addr),
        };
    },
    read_ram: |rom: &Cartridge, addr: u16| -> u8 { rom.ram_data[(addr - 0xa000) as usize] },
    write_ram: |rom: &mut Cartridge, addr: u16, value: u8| {
        rom.ram_data[(addr - 0xa000) as usize] = value;
    },
};

pub static MBC1: Mbc = Mbc {
    name: "MBC1",
    read_rom: |rom: &Cartridge, addr: u16| -> u8 {
        match addr & 0xf000 {
            0x0000 | 0x1000 | 0x2000 | 0x3000 => rom.rom_data[addr as usize],
            0x4000 | 0x5000 | 0x6000 | 0x7000 => *rom
                .rom_data
                .get(rom.rom_offset + (addr - 0x4000) as usize)
                .unwrap_or(&0x0),
            _ => {
                warnln!("Reading from unknown Cartridge ROM location 0x{:04x}", addr);
                0xff
            }
        }
    },
    write_rom: |rom: &mut Cartridge, addr: u16, value: u8| {
        match addr & 0xf000 {
            // RAM enabled flag
            0x0000 | 0x1000 => {
                rom.ram_enabled = (value & 0x0f) == 0x0a;
            }
            // ROM bank selection 5 lower bits
            0x2000 | 0x3000 => {
                let mut rom_bank = value & 0x1f;
                rom_bank &= (rom.rom_bank_count * 2 - 1) as u8;
                if rom_bank == 0 {
                    rom_bank = 1;
                }
                rom.set_rom_bank(rom_bank);
            }
            // RAM bank selection and ROM bank selection upper bits
            0x4000 | 0x5000 => {
                let ram_bank = value & 0x03;
                if ram_bank as u16 >= rom.ram_bank_count {
                    return;
                }
                rom.set_ram_bank(ram_bank);
            }
            // ROM mode selection
            0x6000 | 0x7000 => {
                if value == 0x1 && rom.rom_bank_count > 32 {
                    unimplemented!("Advanced ROM banking mode for MBC1 is not implemented");
                }
            }
            _ => warnln!("Writing to unknown Cartridge ROM location 0x{:04x}", addr),
        }
    },
    read_ram: |rom: &Cartridge, addr: u16| -> u8 {
        if !rom.ram_enabled {
            return 0xff;
        }
        rom.ram_data[rom.ram_offset + (addr - 0xa000) as usize]
    },
    write_ram: |rom: &mut Cartridge, addr: u16, value: u8| {
        if !rom.ram_enabled {
            warnln!("Attempt to write to ERAM while write protect is active");
            return;
        }
        rom.ram_data[rom.ram_offset + (addr - 0xa000) as usize] = value;
    },
};

pub static MBC3: Mbc = Mbc {
    name: "MBC3",
    read_rom: |rom: &Cartridge, addr: u16| -> u8 {
        match addr & 0xf000 {
            0x0000 | 0x1000 | 0x2000 | 0x3000 => rom.rom_data[addr as usize],
            0x4000 | 0x5000 | 0x6000 | 0x7000 => *rom
                .rom_data
                .get(rom.rom_offset + (addr - 0x4000) as usize)
                .unwrap_or(&0x0),
            _ => {
                warnln!("Reading from unknown Cartridge ROM location 0x{:04x}", addr);
                0xff
            }
        }
    },
    write_rom: |rom: &mut Cartridge, addr: u16, value: u8| {
        match addr & 0xf000 {
            // RAM enabled flag
            0x0000 | 0x1000 => {
                rom.ram_enabled = (value & 0x0f) == 0x0a;
            }
            // ROM bank selection
            0x2000 | 0x3000 => {
                let mut rom_bank = value & 0x7f;
                rom_bank &= (rom.rom_bank_count * 2 - 1) as u8;
                if rom_bank == 0 {
                    rom_bank = 1;
                }
                rom.set_rom_bank(rom_bank);
            }
            // RAM bank selection
            0x4000 | 0x5000 => {
                let ram_bank = value & 0x03;
                if ram_bank as u16 >= rom.ram_bank_count {
                    return;
                }
                rom.set_ram_bank(ram_bank);
            }
            _ => warnln!("Writing to unknown Cartridge ROM location 0x{:04x}", addr),
        }
    },
    read_ram: |rom: &Cartridge, addr: u16| -> u8 {
        if !rom.ram_enabled {
            return 0xff;
        }
        rom.ram_data[rom.ram_offset + (addr - 0xa000) as usize]
    },
    write_ram: |rom: &mut Cartridge, addr: u16, value: u8| {
        if !rom.ram_enabled {
            warnln!("Attempt to write to ERAM while write protect is active");
            return;
        }
        rom.ram_data[rom.ram_offset + (addr - 0xa000) as usize] = value;
    },
};

pub static MBC5: Mbc = Mbc {
    name: "MBC5",
    read_rom: |rom: &Cartridge, addr: u16| -> u8 {
        match addr & 0xf000 {
            0x0000 | 0x1000 | 0x2000 | 0x3000 => rom.rom_data[addr as usize],
            0x4000 | 0x5000 | 0x6000 | 0x7000 => *rom
                .rom_data
                .get(rom.rom_offset + (addr - 0x4000) as usize)
                .unwrap_or(&0x0),
            _ => {
                warnln!("Reading from unknown Cartridge ROM location 0x{:04x}", addr);
                0xff
            }
        }
    },
    write_rom: |rom: &mut Cartridge, addr: u16, value: u8| {
        match addr & 0xf000 {
            // RAM enabled flag
            0x0000 | 0x1000 => {
                rom.ram_enabled = (value & 0x0f) == 0x0a;
            }
            // ROM bank selection
            0x2000 => {
                let rom_bank = value;
                rom.set_rom_bank(rom_bank);
            }
            // RAM bank selection
            0x4000 | 0x5000 => {
                let ram_bank = value & 0x0f;
                if ram_bank as u16 >= rom.ram_bank_count {
                    return;
                }
                rom.set_ram_bank(ram_bank);
            }
            _ => warnln!("Writing to unknown Cartridge ROM location 0x{:04x}", addr),
        }
    },
    read_ram: |rom: &Cartridge, addr: u16| -> u8 {
        if !rom.ram_enabled {
            return 0xff;
        }
        rom.ram_data[rom.ram_offset + (addr - 0xa000) as usize]
    },
    write_ram: |rom: &mut Cartridge, addr: u16, value: u8| {
        if !rom.ram_enabled {
            warnln!("Attempt to write to ERAM while write protect is active");
            return;
        }
        rom.ram_data[rom.ram_offset + (addr - 0xa000) as usize] = value;
    },
};
