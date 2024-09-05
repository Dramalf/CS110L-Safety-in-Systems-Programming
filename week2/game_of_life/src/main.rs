use nannou::prelude::*;
use rand::Rng;
use rdiff::grid::Grid;
use std::env;
use std::ops::Add;
use std::process;

const CELL_SIZE: f32 = 10.0; // 定义每个细胞的大小

fn to_int(s: &String) -> usize {
    s.parse::<usize>().unwrap_or(20)
}
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Too few arguments.");
        process::exit(1);
    }
    let size = to_int(&args[1]);
    let speed = to_int(&args[2]);
    // let model = Model::new(size, speed);
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}

#[derive(Debug)]
struct Model {
    board: Grid,
    speed: usize,
}
impl Model {
    fn new(size: usize, speed: usize) -> Model {
        let mut board = Grid::new(size, size);
        board.randomize();
        Model { board, speed }
    }
}
trait randomize {
    fn randomize(&mut self);
}
impl randomize for Grid{
    fn randomize(&mut self) {
        let (rows, cols) = self.size();
        let mut rng = rand::thread_rng(); // 创建一个随机数生成器
        for i in 0..rows {
            for j in 0..cols {
                let _ = self.set(
                    i,
                    j,
                    match rng.gen_range(0..=10) {
                        1 => 1,
                        _ => 0,
                    },
                );
            }
        }
    }
}


fn count_live_neighbors(x: usize, y: usize, grid:&Grid) -> usize {
    let mut count = 0;
    let (rows, cols) = grid.size();

    for dx in [-1, 0, 1].iter().cloned() {
        for dy in [-1, 0, 1].iter().cloned() {
            if dx == 0 && dy == 0 {
                continue; // 跳过自身
            }

            let nx = x as isize + dx;
            let ny = y as isize + dy;

            if nx >= 0 && ny >= 0 && nx < rows as isize && ny < rows as isize {
                if grid.get(nx as usize, ny as usize) == Some(1) {
                    count += 1;
                }
            }
        }
    }

    count
}
fn model(app:&App)->Model{
    Model::new(40, 10)
}
fn update(app: &App, model: &mut Model, _update: Update) {
    let mut board = &mut model.board;
    let (rows, cols) = board.size();

    for x in 0..rows {
        for y in 0..cols {
            let live_neighbors: usize = count_live_neighbors(x, y, &mut board);
            let cur_status=board.get(x, y).unwrap();
            let next_status=match (cur_status,live_neighbors) {
                (0,3)|(1,2)|(1,3) =>1,
                _=>0,
            };
            let _ = board.set(x, y, next_status);
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let (rows, cols) = model.board.size();
    let draw = app.draw();
    draw.background().color(BLACK);

    for x in 0..rows {
        for y in 0..cols {
            let color = match model.board.get(x, y) {
                Some(1) => WHITE,
                _ => BLACK,
            };
            let x_pos = x as f32 * CELL_SIZE - (rows as f32 * CELL_SIZE / 2.0);
            let y_pos = y as f32 * CELL_SIZE - (cols as f32 * CELL_SIZE / 2.0);

            draw.rect()
                .color(color)
                .x_y(x_pos, y_pos)
                .w_h(CELL_SIZE, CELL_SIZE);
        }
    }

    draw.to_frame(app, &frame).unwrap();
}
