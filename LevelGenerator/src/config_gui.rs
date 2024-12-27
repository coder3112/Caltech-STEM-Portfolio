use notan::egui::*;
use notan::prelude::AppState;

#[derive(AppState)]
pub struct State {
    pub size: u16,
    pub use_seed: bool,
    pub use_automata: bool,
    pub fill_percent: f64,
    pub live_cells_required: u8,
    pub step: u8,
    pub seed: String,
    pub use_room_placement: bool,
}

pub fn sidepanel(ctx: &Context, state: &mut State) -> f32 {
    let mut width_sidepanel = 0.0;
    
    Window::new("Config")
        .resizable(true)
        .frame(Frame {
            fill: Color32::from_rgb(36,36,41),
            outer_margin: style::Margin::symmetric(0.0,0.0),
            inner_margin: style::Margin::symmetric(10.0,25.0),
            ..Default::default()
        })
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Configuration Options");
            });

            ui.separator();
            ui.add_space(20.);

            ui.add(Slider::new(&mut state.size, 16..=1024).text("Size").clamp_to_range(true).smart_aim(true));

            ui.checkbox(&mut state.use_seed, "use seed");
            ui.label("This will use a seed that gives a consistent looking result across sizes as long as the seed is the same.");
            if state.use_seed {
                ui.text_edit_singleline(&mut state.seed);
            }
            ui.checkbox(&mut state.use_automata, "Algorithm 2: Cellular Automata");
            ui.label("This uses cellular automata to generate cave-like looing structures. Observe how this is much more organic and natural compred to Algorithm 1 (Room Placement)");
            if state.use_automata {
                ui.add(Slider::new(&mut state.live_cells_required, 2..=6).text("How many cells must be alive? Read how the algorithm works above for more information. Basically determines how dense/sparse the placement would be.").clamp_to_range(true).smart_aim(true).prefix("Live Cells Required"));
                ui.add(Slider::new(&mut state.step, 2..=12).text("how many iterations of checking and updating? Change in conjunction to live cells required for balanced affect.").clamp_to_range(true).smart_aim(true).prefix("Iterations"));
                ui.add(Slider::new(&mut state.fill_percent, 0.25..=0.75).text("how many cells should be filled initially before iterations, affects how empty/full the level looks").clamp_to_range(true).smart_aim(true).step_by(0.05).fixed_decimals(2).prefix("Fill percent"));

            }

            width_sidepanel = ui.available_width();
        });
    width_sidepanel
}
