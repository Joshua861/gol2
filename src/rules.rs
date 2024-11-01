use std::{
    f64::consts,
    ops::RangeBounds,
    time::{SystemTime, UNIX_EPOCH},
};

use macroquad::rand::{gen_range, rand as rand_mq};
use rand::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    board::{Board, Tile},
    config::Config,
};

macro_rules! rulestring {
    ($s: expr) => {{
        let bytes = $s.as_bytes();
        let mut i = 0;
        let mut survive = [false; 9];
        let mut spawn = [false; 9];
        let mut parsing_survive = true;

        while i < bytes.len() {
            match bytes[i] as char {
                '/' => {
                    parsing_survive = false;
                    i += 1;
                    continue;
                }
                c @ '0'..='8' => {
                    let idx = char_to_digit!(c);
                    if parsing_survive {
                        survive[idx] = true;
                    } else {
                        spawn[idx] = true;
                    }
                }
                _ => {}
            }
            i += 1;
        }

        Rule::Rulestring(Rulestring { survive, spawn })
    }};
}

macro_rules! char_to_digit {
    ($c: expr) => {
        match $c {
            '0' => 0,
            '1' => 1,
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            _ => panic!("Invalid digit"),
        }
    };
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum Rule {
    Rulestring(Rulestring),
    Custom(usize),
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Copy)]
pub struct Rulestring {
    pub survive: [bool; 9],
    pub spawn: [bool; 9],
}

pub const CONWAY: Rule = rulestring!("23/3");
pub const MAZE: Rule = rulestring!("12345/3");
pub const MAZE_MICE: Rule = rulestring!("12345/37");
pub const FALLING_STARS: Rule = Rule::Custom(0);
pub const MAZE_CYCLE: Rule = Rule::Custom(1);
pub const NOISE: Rule = Rule::Custom(2);
pub const WORLEY_LINES: Rule = Rule::Custom(3);
pub const SPACE: Rule = Rule::Custom(4);
pub const PERLIN_NOISE: Rule = Rule::Custom(5);

pub const RULES: &[(&str, Rule)] = &[
    ("Conway", CONWAY),
    ("Maze", MAZE),
    ("Maze with mice", MAZE_MICE),
    ("Falling stars", FALLING_STARS),
    ("Maze cycle", MAZE_CYCLE),
    ("Noise", NOISE),
    ("Worley noise", WORLEY_LINES),
    ("Space", SPACE),
    ("Perlin noise", PERLIN_NOISE),
];

type CustomRule = fn(&mut Board, &Config);

pub const CUSTOM_RULES: &[CustomRule] = &[
    falling_stars,
    maze_cycle,
    noise,
    worley,
    space,
    perlin_noise,
];

fn falling_stars(board: &mut Board, config: &Config) {
    // {
    //     let s = (board.width() + board.height()) / 10;
    //
    //     for _ in 0..s {
    //         let y = gen_range(0., board.height() as f32) as isize;
    //         let x = gen_range(0., board.width() as f32) as isize;
    //
    //         board.set(x, y, true);
    //     }
    // }

    for y in 0..board.height() {
        for x in 0..board.width() {
            let (x, y) = (x as isize, y as isize);
            if rand_mq() % 40 == 0 {
                board.set(x, y, false);
            }
        }
    }

    let old = board.clone();

    for y in 0..board.height() {
        for x in 0..board.width() {
            let (x, y) = (x as isize, y as isize);

            if rand_mq() % 2000 == 0 {
                board.set(x, y, true);
            }

            let t = old.get(x, y);

            if t.alive() {
                if !old.get(x, y + 1).alive() {
                    board.set(x, y + 1, true);
                    board.set(x, y, false);
                } else {
                    board.set(x, y, false);
                }
            }

            board.get_mut(x, y).update_heat(config);

            let t = board.get_mut(x, y);
            t.heat = t.heat.saturating_sub(10);
        }
    }
}

fn maze_cycle(board: &mut Board, _config: &Config) {
    {
        let x = board.width() / 2;
        let y = board.height() / 2;

        for y in y - 2..y + 2 {
            for x in x - 2..x + 2 {
                if rand_mq() % 4 == 0 {
                    board.set_u(x, y, true);
                }
            }
        }
    }

    if let Rule::Rulestring(rule) = MAZE {
        let old = board.clone();

        for y in 0..board.height() {
            for x in 0..board.width() {
                let (x, y) = (x as isize, y as isize);
                let n = old.count_neighbors(x, y);
                let is_alive = old.is_alive(x, y);

                if is_alive {
                    let tile = board.get_mut(x, y);
                    tile.alive = rule.survive[n];
                    tile.heat = tile.heat.saturating_add(5);

                    if tile.heat == 255 {
                        board.set(x, y, false);
                    }
                } else {
                    let tile = board.get_mut(x, y);
                    tile.alive = rule.spawn[n] && tile.heat < 100;

                    tile.heat = tile.heat.saturating_sub(1);
                }
            }
        }
    } else {
        panic!("What the fuck");
    }
}

fn noise(board: &mut Board, config: &Config) {
    let s = 500;

    for y in 0..board.height() {
        for x in 0..board.width() {
            let tile = board.get_mut_u(x, y);

            if tile.alive {
                tile.alive = rand_mq() % s != 0
            } else {
                tile.alive = rand_mq() % s == 0
            }

            tile.update_heat(config);
        }
    }
}

fn perlin_noise(mut board: &mut Board, _config: &Config) {
    struct Perlin4D {
        perm: Vec<usize>,
        grad4: [[f64; 4]; 32],
    }

    impl Perlin4D {
        fn new(seed: u64) -> Self {
            let mut rng = StdRng::seed_from_u64(seed);

            let mut perm = (0..256).collect::<Vec<_>>();
            perm.shuffle(&mut rng);
            perm.extend(perm.clone());

            let mut grad4 = [[0.0; 4]; 32];
            for g in grad4.iter_mut() {
                let len = loop {
                    for i in 0..4 {
                        g[i] = rng.gen_range(-1.0..=1.0);
                    }
                    let len_sq: f64 = g.iter().map(|x| x * x).sum();
                    if len_sq > 0.0 {
                        break len_sq.sqrt();
                    }
                };

                for i in 0..4 {
                    g[i] /= len;
                }
            }

            Self { perm, grad4 }
        }

        fn grad4_dot(&self, hash: usize, x: f64, y: f64, z: f64, w: f64) -> f64 {
            let g = &self.grad4[hash & 31];
            g[0] * x + g[1] * y + g[2] * z + g[3] * w
        }

        fn fade(t: f64) -> f64 {
            t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
        }

        // fn lerp(a: f64, b: f64, t: f64) -> f64 {
        //     a + t * (b - a)
        // }

        fn noise(&self, x: f64, y: f64, z: f64, w: f64) -> f64 {
            let x0 = x.floor() as i32;
            let y0 = y.floor() as i32;
            let z0 = z.floor() as i32;
            let w0 = w.floor() as i32;

            let x = x - x0 as f64;
            let y = y - y0 as f64;
            let z = z - z0 as f64;
            let w = w - w0 as f64;

            let x0 = x0.rem_euclid(256) as usize;
            let y0 = y0.rem_euclid(256) as usize;
            let z0 = z0.rem_euclid(256) as usize;
            let w0 = w0.rem_euclid(256) as usize;

            let u = Self::fade(x);
            let v = Self::fade(y);
            let t = Self::fade(z);
            let s = Self::fade(w);

            let a = self.perm[x0] + y0;
            // let aa = self.perm[a] + z0;
            // let ab = self.perm[a + 1] + z0;
            let b = self.perm[x0 + 1] + y0;
            // let ba = self.perm[b] + z0;
            // let bb = self.perm[b + 1] + z0;

            let mut result = 0.0;

            for i in 0..2 {
                for j in 0..2 {
                    for k in 0..2 {
                        for l in 0..2 {
                            let ix = x0 + i;
                            let iy = y0 + j;
                            let iz = z0 + k;
                            let iw = w0 + l;

                            let fx = x - i as f64;
                            let fy = y - j as f64;
                            let fz = z - k as f64;
                            let fw = w - l as f64;

                            let hash =
                                self.perm[self.perm[self.perm[self.perm[ix] + iy] + iz] + iw];
                            let grad = self.grad4_dot(hash, fx, fy, fz, fw);

                            let weight = (1.0 - i as f64 - u)
                                * (1.0 - j as f64 - v)
                                * (1.0 - k as f64 - t)
                                * (1.0 - l as f64 - s);

                            result += grad * weight;
                        }
                    }
                }
            }

            result * 1.5
        }
    }

    fn generate_noise_grid(
        perlin: &Perlin4D,
        width: usize,
        height: usize,
        scale: f64,
        z: f64,
        w: f64,
    ) -> Vec<Vec<f64>> {
        let mut grid = vec![vec![0.0; width]; height];

        for y in 0..height {
            for x in 0..width {
                let nx = x as f64 * scale;
                let ny = y as f64 * scale;
                grid[y][x] = perlin.noise(nx, ny, z, w);
            }
        }

        grid
    }

    fn get_time() -> f64 {
        let start = SystemTime::now();
        let since_epoch = start.duration_since(UNIX_EPOCH).unwrap();

        since_epoch.as_secs_f64() / 5.
    }

    // let old = board.clone();

    // board.clear();

    let perlin = Perlin4D::new(0);
    let width = board.width();
    let height = board.height();
    let scale = 0.005;
    let z = 0.;
    let w = get_time();

    let noise_grid = generate_noise_grid(&perlin, width, height, scale, z, w);

    for x in 0..board.width() {
        for y in 0..board.height() {
            let noise_value = noise_grid[y][x].abs();

            dither(&mut board, x, y, noise_value);

            // let t = board.get_mut_u(x, y);
            // if t.alive {
            //     t.heat = 255;
            // } else {
            //     t.heat = t.heat.saturating_sub(1);
            // }
            //
            // if t.heat < 235 {
            //     t.heat = 0;
            // }
        }
    }
}

fn dither(board: &mut Board, x: usize, y: usize, value: f64) {
    const ONE: f64 = 0.1;
    const TWO: f64 = 0.2;
    const THREE: f64 = 0.3;
    const FOUR: f64 = 0.4;
    const FIVE: f64 = 0.52;
    const SIX: f64 = 0.64;

    if (0.0..ONE).contains(&value) {
        board.set_u(x, y, false);
    } else if (ONE..TWO).contains(&value) {
        if x % 3 == 0 && y % 3 == 0 {
            board.set_u(x, y, true);
        } else {
            board.set_u(x, y, false);
        }
    } else if (TWO..THREE).contains(&value) {
        if x % 2 == 0 && y % 2 == 0 {
            board.set_u(x, y, true);
        } else {
            board.set_u(x, y, false);
        }
    } else if (THREE..FOUR).contains(&value) {
        if x % 2 == y % 2 {
            board.set_u(x, y, true);
        } else {
            board.set_u(x, y, false);
        }
    } else if (FOUR..FIVE).contains(&value) {
        if !(x % 2 == 0 && y % 2 == 0) {
            board.set_u(x, y, true);
        } else {
            board.set_u(x, y, false);
        }
    } else if (FIVE..SIX).contains(&value) {
        if !(x % 3 == 0 && y % 3 == 0) {
            board.set_u(x, y, true);
        } else {
            board.set_u(x, y, false);
        }
    } else if (SIX..1.0).contains(&value) {
        board.set_u(x, y, true);
    } else {
        dbg!(value, x, y);
    }
}

fn worley(mut board: &mut Board, _config: &Config) {
    board.clear();

    // about one point in every 20x20 area.
    let num_points = (board.width() * board.height()) / 20usize.pow(2);

    let points: Vec<_> = vec![false; num_points]
        .iter()
        .map(|_| {
            let x = gen_range(0, board.width());
            let y = gen_range(0, board.height());

            (x, y)
        })
        .collect();

    let mut grid = vec![(0, 0); board.width() * board.height()];

    for y in 0..board.height() {
        for x in 0..board.width() {
            let mut min_dist = f32::MAX;
            let mut closest_point: Option<&(usize, usize)> = None;

            for point in points.iter() {
                let dist = (x as f32 - point.0 as f32).abs().powi(2)
                    + (y as f32 - point.1 as f32).abs().powi(2);

                if dist < min_dist {
                    min_dist = dist;
                    closest_point = Some(point);
                }
            }

            // board.get_mut_u(x, y).heat = 255 - (min_dist * 10.) as u8;
            dither(board, x, y, (min_dist as f64 / 500.).clamp(0., 1.));

            grid[y * board.width() + x] = *closest_point.unwrap();
        }
    }

    for x in 0..board.width() {
        for y in 0..board.height() {
            let point = grid[y * board.width() + x];

            {
                let i = (y + 1) * board.width() + x;

                if x < board.width() - 1 && grid.len() > i && grid[i] != point {
                    board.set_u(x + 1, y, true);
                }
            }

            {
                let i = y * board.width() + x + 1;

                if y < board.height() - 1 && grid.len() > i && grid[i] != point {
                    board.set_u(x + 1, y, true);
                }
            }
        }
    }
}

fn space(board: &mut Board, config: &Config) {
    let height = board.height();
    let width = board.width();

    let mut moves = Vec::new();

    for y in 0..height {
        for x in 0..width {
            let tile = board.get_mut_u(x, y);
            if tile.alive {
                if rand_mq() % 2 == 0 {
                    if y + 1 < height
                        && !board.get_u(x, y + 1).alive
                        && y > 0
                        && (board.get_u(x, y - 1).alive || rand_mq() % 10 == 0)
                    {
                        moves.push((x, y, x, y + 1));
                    } else if y + 1 < height
                        && (board.get_u(x, y + 1).alive || rand_mq() % 10 == 0)
                        && y > 0
                        && !board.get_u(x, y - 1).alive
                    {
                        moves.push((x, y, x, y - 1));
                    }
                } else {
                    if x + 1 < width
                        && !board.get_u(x + 1, y).alive
                        && x > 0
                        && (board.get_u(x - 1, y).alive || rand_mq() % 10 == 0)
                    {
                        moves.push((x, y, x + 1, y));
                    } else if x + 1 < width
                        && (board.get_u(x + 1, y).alive || rand_mq() % 10 == 0)
                        && x > 0
                        && !board.get_u(x - 1, y).alive
                    {
                        moves.push((x, y, x - 1, y));
                    }
                }
            }
        }
    }

    for (from_x, from_y, to_x, to_y) in moves {
        let from_tile = board.get_mut_u(from_x, from_y);
        from_tile.alive = false;
        from_tile.update_heat(config);

        let to_tile = board.get_mut_u(to_x, to_y);
        to_tile.alive = true;
        to_tile.update_heat(config);
    }
}
