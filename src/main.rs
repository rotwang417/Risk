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


// use macroquad::prelude::*;
// use std::fs;
// use std::path::Path;
// use serde::Deserialize;
//
// // Struct representing a territory
// #[derive(Deserialize)]
// struct TerritoryData
// {
//     name: String,
//     vertices: Vec<Vec2>,  // Polygon vertices for the territory shape
//     owner: usize,         // Owner of the territory (player index)
//     armies: i32,
//     selected: bool,       // Whether the territory is selected
// }
//
// struct Territory {
//     name: String,
//     vertices: Vec<Vec2>,
//     owner: usize,
//     armies: i32,
//     selected: bool,
// }
//
// impl TerritoryData {
//     fn to_territory(&self) -> Territory {
//         Territory {
//             name: self.name.clone(),
//             vertices: self.vertices.iter().map(|v| vec2(v[0], v[1])).collect(),
//             owner: self.owner,
//             armies: self.armies,
//             selected: self.selected,
//         }
//     }
// }
//
// impl Territory {
//     // Check if the point (mouse position) is inside the polygon (territory)
//     fn is_point_inside(&self, point: Vec2) -> bool
//     {
//         let mut is_inside = false;
//         let mut j = self.vertices.len() - 1;
//         for i in 0..self.vertices.len() {
//             let vi = &self.vertices[i];
//             let vj = &self.vertices[j];
//
//             // Ray-casting algorithm to detect if point is inside polygon
//             if (vi.y > point.y) != (vj.y > point.y) &&
//                 (point.x < (vj.x - vi.x) * (point.y - vi.y) / (vj.y - vi.y) + vi.x) {
//                 is_inside = !is_inside;
//             }
//             j = i;
//         }
//         is_inside
//     }
//
//     // Draw the territory with a color based on its owner and selection status
//     fn draw(&self) {
//         let color = if self.selected { YELLOW } else { match self.owner
//         {
//             0 => BLUE,
//             1 => GREEN,
//             _ => GRAY,
//             }
//         };
//
//         // Iterate through vertices and draw lines between them
//         let n = self.vertices.len();
//         for i in 0..n {
//             let start = self.vertices[i];
//             let end = self.vertices[(i + 1) % n];  // Loop back to the start for the last line
//             draw_line(start.x, start.y, end.x, end.y, 2.0, color);
//         }
//     }
// }
//
// // Struct representing the game state
// struct GameState
// {
//     territories: Vec<Territory>,
//     selected_territory: Option<usize>,  // Index of the currently selected territory
// }
//
// impl GameState
// {
//     fn new() -> GameState {
//         let territories = load_territories_from_json("territories.json")
//             .into_iter()
//             .map(|data| data.to_territory())
//             .collect();
//
//         GameState {
//             territories,
//             selected_territory: None,
//         }
//     }
//
//     // fn new() -> GameState
//     // {
//     //     let territories = vec![
//     //         Territory
//     //         {
//     //             name: "North America".to_string(),
//     //             vertices: vec![vec2(100.0, 100.0), vec2(200.0, 100.0), vec2(200.0, 200.0), vec2(100.0, 200.0)],
//     //             owner: 0,
//     //             armies: 5,
//     //             selected: false,
//     //         },
//     //         Territory
//     //         {
//     //             name: "South America".to_string(),
//     //             vertices: vec![vec2(300.0, 300.0), vec2(400.0, 300.0), vec2(400.0, 400.0), vec2(300.0, 400.0)],
//     //             owner: 1,
//     //             armies: 3,
//     //             selected: false,
//     //         },
//     //         // Add more territories here based on the Risk map
//     //     ];
//     //
//     //     GameState
//     //     {
//     //         territories,
//     //         selected_territory: None,
//     //     }
//     // }
//
//     fn handle_input(&mut self)
//     {
//         if is_mouse_button_pressed(MouseButton::Left)
//         {
//             let mouse_position = mouse_position().into();
//
//             // To store the newly selected territory index
//             let mut newly_selected_territory: Option<usize> = None;
//
//             for (i, territory) in self.territories.iter_mut().enumerate() {
//                 if territory.is_point_inside(mouse_position) {
//                     newly_selected_territory = Some(i); // Mark the current territory for selection
//                 }
//             }
//
//             // Deselect the previously selected territory if any
//             if let Some(selected) = self.selected_territory {
//                 self.territories[selected].selected = false;
//             }
//
//             // Select the newly clicked territory
//             if let Some(i) = newly_selected_territory {
//                 self.territories[i].selected = true;
//                 self.selected_territory = Some(i);
//             }
//         }
//     }
//
//     // Draw all territories on the map
//     fn draw_map(&self)
//     {
//         for territory in &self.territories
//         {
//             territory.draw();
//         }
//
//         // Display info for selected territory
//         if let Some(selected_index) = self.selected_territory
//         {
//             let selected = &self.territories[selected_index];
//             draw_text(
//                 &format!("Selected: {}", selected.name),
//                 10.0,
//                 20.0,
//                 30.0,
//                 DARKGRAY,
//             );
//             draw_text(
//                 &format!("Armies: {}", selected.armies),
//                 10.0,
//                 50.0,
//                 30.0,
//                 DARKGRAY,
//             );
//         }
//     }
// }
//
// // Load territories from a JSON file
// fn load_territories_from_json<P: AsRef<Path>>(path: P) -> Vec<TerritoryData> {
//     let file_content = fs::read_to_string(path).expect("Failed to read territories.json");
//     serde_json::from_str(&file_content).expect("Failed to parse JSON data")
// }
//
// // // Vec2 struct for coordinates
// // fn vec2(x: f32, y: f32) -> Vec2 {
// //     Vec2 { x, y }
// // }
// //
// // #[derive(Deserialize)]
// // struct Vec2 {
// //     x: f32,
// //     y: f32,
// // }
//
// #[macroquad::main("Interactive Risk Map")]
// async fn main()
// {
//     let mut game_state = GameState::new();
//
//     loop
//     {
//         clear_background(WHITE);
//
//         // Handle player input
//         game_state.handle_input();
//
//         // Draw the map
//         game_state.draw_map();
//
//         next_frame().await;
//     }
// }