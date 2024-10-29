use crate::{config::Config, utils::rand_bool};
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use dirs::data_dir;
use serde::ser::SerializeStruct;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone)]
pub struct Board {
    cells: Vec<Tile>,
    width: usize,
    height: usize,
}

#[derive(Clone, Copy)]
pub struct Tile {
    alive: bool,
    heat: u8,
}

impl Tile {
    fn new() -> Self {
        Self {
            alive: false,
            heat: 0,
        }
    }
    fn update_heat(&mut self, config: &Config) {
        if self.alive {
            if config.soft_heat {
                self.heat = self.heat.saturating_add(config.soft_heat_amount);
            } else {
                self.heat = 255;
            }
        } else {
            self.heat = self.heat.saturating_sub(1);
        }
    }
    fn set(&mut self, to: bool) {
        self.alive = to
    }
    pub fn alive(&self) -> bool {
        self.alive
    }
    pub fn heat(&self) -> u8 {
        self.heat
    }
}

impl Board {
    const SURVIVE_RULE: [bool; 9] = [false, false, true, true, false, false, false, false, false];
    const SPAWN_RULE: [bool; 9] = [false, false, false, true, false, false, false, false, false];

    pub fn new(width: usize, height: usize) -> Self {
        Self {
            cells: vec![Tile::new(); width * height],
            width,
            height,
        }
    }

    pub fn update(&mut self, config: &Config) {
        let old = self.clone();

        for y in 0..self.height {
            for x in 0..self.width {
                let (x, y) = (x as isize, y as isize);
                let n = old.count_neighbors(x, y);
                let is_alive = old.is_alive(x, y);

                if is_alive {
                    self.get_mut(x, y).alive = Self::SURVIVE_RULE[n];
                } else {
                    self.get_mut(x, y).alive = Self::SPAWN_RULE[n];
                }

                if config.enable_heat {
                    self.get_mut(x, y).update_heat(config);
                }
            }
        }
    }

    fn wrap_xy(&self, x: isize, y: isize) -> (usize, usize) {
        let x = ((x + self.width as isize) % self.width as isize) as usize;
        let y = ((y + self.height as isize) % self.height as isize) as usize;

        (x, y)
    }

    fn xy_to_idx(&self, x: usize, y: usize) -> usize {
        x + y * self.width
    }

    pub fn get(&self, x: isize, y: isize) -> Tile {
        let (x, y) = self.wrap_xy(x, y);

        self.cells[self.xy_to_idx(x, y)]
    }

    fn get_mut(&mut self, x: isize, y: isize) -> &mut Tile {
        let (x, y) = self.wrap_xy(x, y);
        let i = self.xy_to_idx(x, y);

        &mut self.cells[i]
    }

    fn get_mut_u(&mut self, x: usize, y: usize) -> &mut Tile {
        let (x, y) = self.wrap_xy(x as isize, y as isize);
        let i = self.xy_to_idx(x, y);

        &mut self.cells[i]
    }

    fn is_alive(&self, x: isize, y: isize) -> bool {
        self.get(x, y).alive
    }

    fn count_neighbors(&self, x: isize, y: isize) -> usize {
        let mut count = 0;
        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue; // skip middle cell
                }
                if self.is_alive(x + dx, y + dy) {
                    count += 1;
                }
            }
        }
        count
    }

    pub fn is_inside(&self, x: isize, y: isize) -> bool {
        x >= 0 && x < self.width as isize && y >= 0 && y < self.height as isize
    }

    pub fn set_line(
        &mut self,
        x0: isize,
        y0: isize,
        x1: isize,
        y1: isize,
        alive: bool,
    ) -> Option<()> {
        for (x, y) in clipline::Clipline::new(
            ((x0, y0), (x1, y1)),
            ((0, 0), (self.width as isize - 1, self.height as isize - 1)),
        )? {
            let (x, y) = (x as usize, y as usize);
            self.set(x as isize, y as isize, alive);
        }
        Some(())
    }

    pub fn set(&mut self, x: isize, y: isize, to: bool) {
        self.get_mut(x, y).alive = to;
    }

    pub fn set_u(&mut self, x: usize, y: usize, to: bool) {
        self.set(x as isize, y as isize, to);
    }

    pub fn set_line_u(&mut self, x0: usize, y0: usize, x1: usize, y1: usize, to: bool) {
        self.set_line(x0 as isize, y0 as isize, x1 as isize, y1 as isize, to);
    }

    pub fn randomize(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let tile = self.get_mut_u(x, y);
                tile.alive = rand_bool();
                tile.heat = 255;
            }
        }
    }

    pub fn clear(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let tile = self.get_mut_u(x, y);
                tile.alive = false;
                tile.heat = 0;
            }
        }
    }

    pub fn debug_print(&self, iter_count: u64) {
        println!("\n\n---\n");
        println!("{}x{} // {}", self.width, self.height, iter_count);
        for y in 0..self.height {
            for x in 0..self.width {
                let tile = self.get(x as isize, y as isize);
                print!("{}", if tile.alive { "#" } else { " " });
            }
            println!();
        }
        println!("\n---\n\n");
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn saves_dir() -> String {
        format!("{}/gol2/saves", data_dir().unwrap().display())
    }
}

impl Serialize for Board {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut board = self.clone();
        let width = board.width();
        let height = board.height();

        let bytes = bools_to_u8s(board.cells.iter().map(|t| t.alive));
        let base64 = BASE64_STANDARD.encode(&bytes);

        let mut state = serializer.serialize_struct("Board", 4)?;
        state.serialize_field("width", &width)?;
        state.serialize_field("height", &height)?;
        state.serialize_field("cells", &base64)?;
        state.end()
    }
}

fn bools_to_u8s<I>(bool_iter: I) -> Vec<u8>
where
    I: IntoIterator<Item = bool>,
{
    let mut result = Vec::new();
    let mut current_byte = 0;
    let mut bits_set = 0;

    for bool_value in bool_iter.into_iter() {
        current_byte |= (bool_value as u8) << bits_set;
        bits_set += 1;

        if bits_set == 8 {
            result.push(current_byte);
            current_byte = 0;
            bits_set = 0;
        }
    }

    if bits_set > 0 {
        result.push(current_byte);
    }

    result
}

#[test]
fn test_bools_to_u8s() {
    let bools = vec![true, false, true, true, false, false, true, true];
    let result = bools_to_u8s(bools);
    assert_eq!(result, vec![0b10111011]);

    let bools = vec![true; 16];
    let result = bools_to_u8s(bools);
    assert_eq!(result, vec![0xFF, 0xFF]);
}

impl<'de> Deserialize<'de> for Board {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct BoardData {
            width: usize,
            height: usize,
            cells: String,
        }

        let data = BoardData::deserialize(deserializer)?;

        let decoded_cells = BASE64_STANDARD.decode(data.cells.as_bytes()).unwrap();

        let mut tiles = Vec::with_capacity(decoded_cells.len() * 8);
        for byte in decoded_cells {
            for i in 0..8 {
                tiles.push(Tile {
                    alive: (byte & (1 << i)) != 0,
                    heat: 0,
                });
            }
        }

        let mut board = Board::new(data.width, data.height);
        board.cells = tiles;

        Ok(board)
    }
}
