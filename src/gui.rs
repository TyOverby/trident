extern crate lux;
extern crate trident;
extern crate palette;
extern crate rand;
extern crate clamp;

mod ui;

use lux::prelude::*;
use lux::game::*;
use rand::random;
use trident::*;
use lux::interactive::keycodes::VirtualKeyCode;
use std::collections::HashMap;

struct GameWindow {
    keybindings: ui::KeyBindings,
    scale_factor: f32,
    state: GameState,
    camera_pos: (f32, f32),
    player_colors: Vec<(f32, f32, f32, f32)>
}

enum GuiState {
    StarSelected(StarId),
    CarrierSelected(CarrierId),
}

fn random_colors(n: usize) -> Vec<(f32, f32, f32, f32)> {
    use palette::{Hsl, RgbHue};
    use palette::IntoColor;

    let mut r = 0.0f32;
    let mut out = Vec::with_capacity(n);

    for _ in 0 .. n {
        let h = RgbHue::from_radians(r);
        let c = Hsl::new(h, 1.0, 0.5);
        let rgb = c.into_rgb();
        out.push((rgb.red, rgb.green, rgb.blue, 1.0));
        r += 2.4;
    }
    out
}

impl Game for GameWindow {
    fn clear_color(&self) -> Option<[f32; 4]> {
        Some([0.02, 0.02, 0.02, 1.0])
    }

    fn show_fps(&self, _: &Window) -> bool { false }

    fn render(&mut self, dt: f32, window: &mut Window, frame: &mut Frame) -> LuxResult<()> {
        frame.translate(self.camera_pos.0, self.camera_pos.1);
        frame.scale(self.scale_factor, self.scale_factor);

        fn pos_mod(xy: f32, size: f32) -> f32 {
            xy - size / 2.0
        }

        for star in self.state.stars.values() {
            let color = self.player_colors.get(star.owned_by.0 as usize).cloned().unwrap();
            let color_outline = (color.0, color.1, color.2, 0.1);
            let size = star.size as f32 * 1.0;
            let outline = 10.0;

            frame.circle(pos_mod(star.location.0 as f32, size + outline),
                         pos_mod(star.location.1 as f32, size + outline), size + outline)
                 .color(color_outline)
                 .fill();

            frame.circle(pos_mod(star.location.0 as f32, size),
                         pos_mod(star.location.1 as f32, size), size)
                 .color(color)
                 .fill();
        }

        Ok(())
    }

    fn update(&mut self, dt: f32, window: &mut Window, events: &mut EventIterator) -> LuxResult<()>{
        ui::update_camera(&mut self.camera_pos, &mut self.scale_factor, window, &self.keybindings);

        for event in events {
            match event {
                _ => {}
            }
        }

        Ok(())
    }
}


fn main() {
    use rand::Rng;
    let mut state = GameState::new();
    let mut random = rand::thread_rng();

    for i in 0 .. 5 {
        let x = random.gen_range(0, 1000);
        let y = random.gen_range(0, 1000);
        state.stars.insert(StarId(i), Star {
            id: StarId(i),
            owned_by: PlayerId(i),
            location: (x, y),
            size: clamp::clamp(0, random.gen(), 75) + 30,
            economy: 0,
            infrastructure: 0,
            science: 0,
        });
    }
    let mut colors = random_colors(state.stars.len());

    let mut bindings = HashMap::new();
    bindings.insert(ui::Action::CameraUp, vec![VirtualKeyCode::W]);

    let game = GameWindow {
        keybindings: bindings,
        scale_factor: 1.0,
        state: state,
        camera_pos: (0.0, 0.0),
        player_colors: colors,
    };

    game.run_until_end();
}
