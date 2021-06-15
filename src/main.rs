extern crate piston_window;
extern crate rand;

use piston_window::*;
use rand::prelude::*;

fn main() {
    let window_size: f64 = 750.0;
    let square_size: f64 = 10.0;
    let alive_threshold = 35;

    let white = [1.0, 1.0, 1.0, 1.0];
    let black = [0.0, 0.0, 0.0, 1.0];

    let mut grid: Vec<Vec<bool>> = vec![];
    let number_of_element = (window_size / square_size) as usize;
    let mut rng = rand::thread_rng();
    grid.reserve(number_of_element);
    for _ in 0..number_of_element {
        let mut row: Vec<bool> = vec![false; number_of_element];
        for i in 0..number_of_element {
            let is_alive = rng.gen_range(0..100) < alive_threshold;
            if is_alive {
                row[i] = true
            }
        }
        grid.push(row)
    }

    let mut window: PistonWindow = WindowSettings::new("Conway's game of life", [window_size, window_size])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut events = Events::new(EventSettings { max_fps: 16, ups: 16, ups_reset: 0, swap_buffers: true, bench_mode: false, lazy: false});

    while let Some(e) = events.next(&mut window) {
        if let Some(_) = e.render_args() {
            window.draw_2d(&e, |context, graphics, _device| {
                clear([1.0; 4], graphics);

                for (row_index, row) in grid.iter().enumerate() {
                    for (col_index, col) in row.iter().enumerate() {
                        let color = if *col { black } else { white };
                        rectangle(color, [square_size * (row_index as f64), square_size * (col_index as f64), square_size, square_size], context.transform, graphics);
                    }
                }
            });
        }

        if let Some(_) = e.update_args() {
            grid = next(grid);
        }
    }
}

fn next(grid: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let grid_size = grid.len();

    let mut new_grid = vec![];
    new_grid.reserve(grid_size);

    for i in 0..grid_size {
        let mut row = vec![false; grid_size];
        for j in 0..grid_size {
            let mut count = 0;
            let indexes: [(isize, isize); 8] = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];

            for (x, y) in indexes.iter() {
                let xpos = i as isize + x;
                let ypos = j as isize + y;
                if (xpos >= 0) && (ypos >= 0) && (xpos < grid_size as isize) && (ypos < grid_size as isize)  {
                    if grid[xpos as usize][ypos as usize] {
                        count = count + 1;
                    }
                }
            }

            let current = grid[i][j];
            //println!("[{} - {}]: {} - {}", i, j, current, count);
            if current && (count == 2 || count == 3) {
                row[j] = true;
            } else if !current && (count == 3) {
                row[j] = true;
            } else {
                row[j] = false;
            }
        }
        new_grid.push(row)
    }

    return new_grid;
}
