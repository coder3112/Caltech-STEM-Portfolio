use std::borrow::Borrow;
use notan::draw::{CreateDraw, DrawShapes};
use notan::egui::*;
use notan::log::debug;
use notan::math::Mat3;
use notan::prelude::*;
use rand::prelude::*;
use sha2::{Digest, Sha256};

use crate::algorithms::cellular_automata::{reset_automata, step};
use crate::algorithms::*;
use crate::config_gui::{sidepanel, State};
use crate::level::Level;

fn create_hash(text: &str) -> String {
    let mut hasher = Sha256::default();
    hasher.update(text.as_bytes());
    format!("{:x}", hasher.finalize())
}

pub fn setup(app: &mut App) -> State {
    State {
        size: 64,
        use_seed: false,
        seed: "".to_string(),
        live_cells_required: 5,
        step: 4,
        use_automata: false,
        fill_percent: 0.50,
        use_room_placement: true
    }
}

pub fn draw(app: &mut App, gfx: &mut Graphics, plugins: &mut Plugins, state: &mut State) {
    let mut draw = gfx.create_draw();
    let mut width_sidepanel = 0.0;
    let mut output = plugins.egui(|ctx| {
        width_sidepanel = sidepanel(ctx, state);
    });
    let width_sidepanel = width_sidepanel; // discard mut
    debug!("width{}", width_sidepanel);
    let win_size = gfx.size();
    let win_width = win_size.0 as f32;
    let win_height = win_size.1 as f32;
    let tile_size = ((win_width - width_sidepanel).min(win_height) / state.size as f32).powf(0.5);
    debug!("win: {} {}, tile: {}", win_width, win_height, tile_size);
    draw.transform()
        .push(Mat3::from_translation(notan::math::vec2(
            (win_width - width_sidepanel - (tile_size * state.size as f32)) / 2.,
            (win_height - (tile_size * state.size as f32)) / 2.,
        )));
    let mut rng;
    if state.use_seed {
        let hash = create_hash(&state.seed[..]);
        let seed = array_ref!(hash.as_bytes(), 0, 32);
        rng = SeedableRng::from_seed(*seed);
    } else {
        rng = SeedableRng::from_rng(thread_rng()).unwrap();
    }
    let mut data: Vec<Vec<u8>> = Vec::new();
    let mut width = 64;
    let mut height = 64;

    if state.use_room_placement {
        let mut level = room_placement::LevelBSP::new(state.size);
        level.place_rooms(100,4,80,5,120,&mut rng);
        level.place_corridors(&mut rng);
        data = level.level.data;
        width = level.level.width;
        height = level.level.height;
    }

    if state.use_automata {
        let mut level = reset_automata(state.size, &mut rng, state.fill_percent);
        for _ in 0..state.step { step(&mut level, state.live_cells_required); }
        data = level.data;
        width = level.width;
        height = level.height;
    }
    debug!("{:?}", data);
    for row in 1..width as usize {
        for col in 1..height as usize {
            draw.rect(
                (tile_size * (col as f32), 0. + tile_size * (row as f32)),
                (tile_size, tile_size),
            )
            .color([Color::BLUE, Color::WHITE][data[row][col] as usize]);
        }
    }

    output.clear_color(Color::BLACK);
    if output.needs_repaint() {
        gfx.render(&output);
        gfx.render(&draw);
    }
}
