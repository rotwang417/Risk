use macroquad::prelude::*;
use serde::Deserialize;
use std::fs;
use std::path::Path;

// Struct for deserializing JSON data
#[derive(Deserialize)]
struct TerritoryData {
    name: String,
    vertices: Vec<[f32; 2]>,
    owner: usize,
    armies: i32,
    selected: bool,
}

impl TerritoryData {
    fn to_territory(&self) -> Territory {
        Territory {
            name: self.name.clone(),
            vertices: self.vertices.iter().map(|v| vec2(v[0], v[1])).collect(),
            owner: self.owner,
            armies: self.armies,
            selected: self.selected,
        }
    }
}

// Struct representing a territory
struct Territory {
    name: String,
    vertices: Vec<Vec2>,
    owner: usize,
    armies: i32,
    selected: bool,
}

impl Territory {
    fn is_point_inside(&self, point: Vec2) -> bool {
        let mut is_inside = false;
        let mut j = self.vertices.len() - 1;
        for i in 0..self.vertices.len() {
            let vi = &self.vertices[i];
            let vj = &self.vertices[j];

            if (vi.y > point.y) != (vj.y > point.y)
                && (point.x < (vj.x - vi.x) * (point.y - vi.y) / (vj.y - vi.y) + vi.x)
            {
                is_inside = !is_inside;
            }
            j = i;
        }
        is_inside
    }

    fn draw(&self) {
        let color = if self.selected {
            YELLOW
        } else {
            match self.owner {
                0 => BLUE,
                1 => GREEN,
                _ => GRAY,
            }
        };

        let n = self.vertices.len();
        for i in 0..n {
            let start = self.vertices[i];
            let end = self.vertices[(i + 1) % n];
            draw_line(start.x, start.y, end.x, end.y, 2.0, color);
        }
    }
}

struct GameState {
    territories: Vec<Territory>,
    selected_territory: Option<usize>,
}

impl GameState {
    fn new() -> GameState {
        let territories = load_territories_from_json("resources/territories.json")
            .into_iter()
            .map(|data| data.to_territory())
            .collect();

        GameState {
            territories,
            selected_territory: None,
        }
    }

    fn handle_input(&mut self) {
        if is_mouse_button_pressed(MouseButton::Left) {
            let mouse_position = mouse_position().into();

            let mut newly_selected_territory: Option<usize> = None;

            for (i, territory) in self.territories.iter_mut().enumerate() {
                if territory.is_point_inside(mouse_position) {
                    newly_selected_territory = Some(i);
                }
            }

            if let Some(selected) = self.selected_territory {
                self.territories[selected].selected = false;
            }

            if let Some(i) = newly_selected_territory {
                self.territories[i].selected = true;
                self.selected_territory = Some(i);
            }
        }
    }

    fn draw_map(&self) {
        for territory in &self.territories {
            territory.draw();
        }

        if let Some(selected_index) = self.selected_territory {
            let selected = &self.territories[selected_index];
            draw_text(
                &format!("Selected: {}", selected.name),
                10.0,
                20.0,
                30.0,
                DARKGRAY,
            );
            draw_text(
                &format!("Armies: {}", selected.armies),
                10.0,
                50.0,
                30.0,
                DARKGRAY,
            );
        }
    }
}

fn load_territories_from_json<P: AsRef<Path>>(path: P) -> Vec<TerritoryData> {
    let file_content = fs::read_to_string(path).expect("Failed to read territories.json");
    serde_json::from_str(&file_content).expect("Failed to parse JSON data")
}

#[macroquad::main("Interactive Risk Map")]
async fn main() {
    let mut game_state = GameState::new();

    loop {
        clear_background(WHITE);
        game_state.handle_input();
        game_state.draw_map();
        next_frame().await;
    }
}