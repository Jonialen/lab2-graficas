use raylib::prelude::*;
mod framebuffer;
use framebuffer::Framebuffer;

const SCREEN_WIDTH: i32 = 800;
const SCREEN_HEIGHT: i32 = 600;
const CELL_SIZE: i32 = 5;

const GRID_WIDTH: usize = (SCREEN_WIDTH / CELL_SIZE) as usize;
const GRID_HEIGHT: usize = (SCREEN_HEIGHT / CELL_SIZE) as usize;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Game of Life")
        .build();

    let mut framebuffer = Framebuffer::new(SCREEN_WIDTH, SCREEN_HEIGHT, Color::BLACK);
    framebuffer.set_current_color(Color::WHITE);

    let mut grid = [[false; GRID_HEIGHT]; GRID_WIDTH];
    for x in 0..GRID_WIDTH {
        for y in 0..GRID_HEIGHT {
            if rl.get_random_value::<i32>(0..5) == 1 {
                grid[x][y] = true;
            }
        }
    }

    let mut texture = rl.load_texture_from_image(&thread, &Image::gen_image_color(SCREEN_WIDTH, SCREEN_HEIGHT, Color::BLACK)).unwrap();

    rl.set_target_fps(10);

    while !rl.window_should_close() {
        // Update
        let mut next_grid = grid;

        for x in 0..GRID_WIDTH {
            for y in 0..GRID_HEIGHT {
                let neighbors = count_neighbors(&grid, x, y);

                let cell = grid[x][y];
                let new_cell_state = match (cell, neighbors) {
                    // Rule 1: A live cell with fewer than two live neighbours dies, as if by underpopulation.
                    (true, n) if n < 2 => false,
                    // Rule 2: A live cell with two or three live neighbours lives on to the next generation.
                    (true, 2) | (true, 3) => true,
                    // Rule 3: A live cell with more than three live neighbours dies, as if by overpopulation.
                    (true, n) if n > 3 => false,
                    // Rule 4: A dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
                    (false, 3) => true,
                    // All other cells remain in the same state.
                    (otherwise, _) => otherwise,
                };
                next_grid[x][y] = new_cell_state;
            }
        }
        grid = next_grid;

        // Draw
        framebuffer.clear();
        for x in 0..GRID_WIDTH {
            for y in 0..GRID_HEIGHT {
                if grid[x][y] {
                    // Draw a rectangle for each live cell
                    for i in 0..CELL_SIZE {
                        for j in 0..CELL_SIZE {
                            framebuffer.set_pixel(x as i32 * CELL_SIZE + i, y as i32 * CELL_SIZE + j);
                        }
                    }
                }
            }
        }
        
        framebuffer.swap_buffers();

        // Update texture and draw
        let image_colors = framebuffer.get_front_buffer_data();
        let pixel_data: &[u8] = unsafe {
            std::slice::from_raw_parts(
                image_colors.as_ptr() as *const u8,
                (framebuffer.width * framebuffer.height * 4) as usize,
            )
        };
        let _ = texture.update_texture(pixel_data);

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        d.draw_texture(&texture, 0, 0, Color::WHITE);
        d.draw_fps(10, 10);
    }
}

fn count_neighbors(grid: &[[bool; GRID_HEIGHT]; GRID_WIDTH], x: usize, y: usize) -> u8 {
    let mut count = 0;
    for i in -1..=1 {
        for j in -1..=1 {
            if i == 0 && j == 0 {
                continue;
            }

            let col = (x as i32 + i + GRID_WIDTH as i32) % GRID_WIDTH as i32;
            let row = (y as i32 + j + GRID_HEIGHT as i32) % GRID_HEIGHT as i32;
            
            if grid[col as usize][row as usize] {
                count += 1;
            }
        }
    }
    count
}