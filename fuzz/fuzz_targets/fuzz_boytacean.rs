#![no_main]

use libfuzzer_sys::fuzz_target;
use boytacean::{
    gb::{GameBoy, GameBoyMode},
};

fuzz_target!(|data: &[u8]| {
    let mut rom = vec![0 as u8; 8196];
    rom[0x100..data.len() + 0x100].copy_from_slice(data);
    let mut game_boy = GameBoy::new(Some(GameBoyMode::Dmg));
    game_boy.set_ppu_enabled(true);
    game_boy.set_apu_enabled(true);
    game_boy.set_dma_enabled(true);
    game_boy.load(true);
    game_boy.load_rom(&rom);
    for _ in 0..2048 {
        game_boy.clock();
    }
});
