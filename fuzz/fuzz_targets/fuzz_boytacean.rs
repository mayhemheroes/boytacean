#![no_main]

use libfuzzer_sys::fuzz_target;
use boytacean::{
    gb::{GameBoy, GameBoyMode},
};

fn convert_to_valid_rom(rom_num: u8) -> u8 {
    match rom_num % 15 {
        0 => 0x00,
        1 => 0x01,
        2 => 0x02,
        3 => 0x03,
        4 => 0x0f,
        5 => 0x10,
        6 => 0x11,
        7 => 0x12,
        8 => 0x13,
        9 => 0x19,
        10 => 0x1a,
        11 => 0x1b,
        12 => 0x1c,
        13 => 0x1d,
        _ => 0x1e,
    }
}

fuzz_target!(|data: &[u8]| {
    let mut rom = vec![0 as u8; 1024 * 256];
    rom[0x00..data.len() + 0x00].copy_from_slice(data);
    rom[0x0147] = convert_to_valid_rom(rom[0x0147]);
    rom[0x0148] = rom[0x0148] % 10;
    rom[0x0149] = rom[0x0149] % 7;
    let mut game_boy = GameBoy::new(Some(GameBoyMode::Dmg));
    game_boy.set_ppu_enabled(true);
    game_boy.set_apu_enabled(true);
    game_boy.set_dma_enabled(true);
    game_boy.set_timer_enabled(true);
    game_boy.load(true);
    game_boy.load_rom(&rom);
    game_boy.cpu().boot();

    for a in 0..1024 {
        // println!("{0}", a);
        game_boy.clock();
    }
});
