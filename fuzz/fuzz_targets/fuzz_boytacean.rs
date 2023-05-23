#![no_main]

use libfuzzer_sys::fuzz_target;
use boytacean::{
    gb::{GameBoy, GameBoyMode},
};

fuzz_target!(|data: &[u8]| {
    let mut rom = vec![0 as u8; 8196];
    rom[0..data.len()].copy_from_slice(data);
    rom[0x0147] = 0;
    let mut game_boy = GameBoy::new(Some(GameBoyMode::Dmg));
    game_boy.load(true);
    game_boy.load_rom(&rom);
    for _ in 0..256 {
        game_boy.clock();
    }
});
