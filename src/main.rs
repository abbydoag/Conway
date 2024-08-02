use minifb::{Window, WindowOptions, Key};
use std::time::Duration;

mod framebuffer;

use crate::framebuffer::Framebuffer;

// Función para contar vecinos vivos
fn count_live_neighbors(framebuffer: &Framebuffer, x: usize, y: usize) -> u32 {
    let mut count = 0;
    let neighbors = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),         (0, 1),
        (1, -1), (1, 0), (1, 1)
    ];
    for &(dx, dy) in &neighbors {
        let nx = (x as isize + dx) as usize;
        let ny = (y as isize + dy) as usize;
        if framebuffer.get(nx, ny) == 0x3D85C6 {
            count += 1;
        }
    }
    count
}

// Función para calcular la siguiente generación
fn next_generation(framebuffer: &Framebuffer, width: usize, height: usize) -> Vec<u32> {
    let mut new_buffer = vec![0; width * height];
    
    for y in 0..height {
        for x in 0..width {
            let live_neighbors = count_live_neighbors(framebuffer, x, y);
            let is_alive = framebuffer.get(x, y) == 0x3D85C6;
            
            let new_state = match (is_alive, live_neighbors) {
                (true, 2) | (true, 3) => 0x3D85C6, // Vive 
                (true, _) => 0x000000,            // Muere
                (false, 3) => 0x3D85C6,           // Resucita
                _ => 0x000000                    // Sigue muerto
            };
            
            new_buffer[y * width + x] = new_state;
        }
    }
    
    new_buffer
}

//varios ptrones
fn initialize_patterns(framebuffer: &mut Framebuffer, patterns: Vec<(usize, usize, Vec<(usize, usize)>)>) {
    framebuffer.clear();
    for (offset_x, offset_y, pattern) in patterns {
        for (x, y) in pattern {
            if x + offset_x < framebuffer.width && y + offset_y < framebuffer.height {
                framebuffer.point(x + offset_x, y + offset_y);
            }
        }
    }
}

fn main() {
    let window_width = 300;
    let window_height = 300;
    let framebuffer_width = 100;
    let framebuffer_height = 100;

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);
    framebuffer.set_current_color(0x3D85C6);
    framebuffer.set_background_color(0x000000);

    let mut window = Window::new(
        "Conway's Game of Life",
        window_width,
        window_height,
        WindowOptions::default(),
    ).unwrap();

    //patrones
    let patterns = vec![
        (0, 10, vec![
            (1, 0), (2, 1), (0, 2), (1, 2), (2, 2) // Glider
        ]),
        (20, 35 , vec![
            (1, 0), (2, 1), (0, 2), (1, 2), (2, 2) // Glider2
        ]),
        (70, 75 , vec![
            (1, 0), (2, 1), (0, 2), (1, 2), (2, 2) // Glider2
        ]),
        (30, 80, vec![
            (2, 1), (2, 2), (2, 3) // Blinker
        ]),
        (0, 70, vec![
            (2, 1), (2, 2), (2, 3) // Blinker 2
        ]),
        (45,76, vec![
            (0, 1), (1, 1), (2, 1), (3, 1), //mwss
            (0, 2), (3, 2),
            (0, 3), (1, 3),
            (2, 4), (3, 4),
            (1, 5), (2, 5) 
        ]),
        (40, 20, vec![
            (2, 1), (3, 1), (4, 1), (5, 1),
            (1, 2), (5, 2),
            (1, 3), (4, 3),
            (1, 4), (5, 4),
            (2, 5), (3, 5), (4, 5) // LWSS
        ]),
        (90, 20, vec![
            (2, 1), (3, 1), (4, 1), (5, 1),
            (1, 2), (5, 2),
            (1, 3), (4, 3),
            (1, 4), (5, 4),
            (2, 5), (3, 5), (4, 5) // LWSS 2
        ]),
        (70, 40, vec![
            (1, 1), (1, 2), (2, 1), (2, 2), // Beacon
            (3, 3), (3, 4), (4, 3), (4, 4)
        ]),
        (33, 70, vec![
            (2, 1), (3, 1), (4, 1), (5, 1), (6, 1),
            (1, 2), (7, 2),
            (1, 3), (7, 3),
            (1, 4), (7, 4),
            (2, 5), (3, 5), (4, 5), (5, 5), (6, 5) // Pentadecathlon
        ]),
        (40, 50, vec![
            (2, 1), (3, 1), (4, 1), (5, 1), (6, 1),
            (1, 2), (7, 2),
            (1, 3), (7, 3),
            (1, 4), (7, 4),
            (2, 5), (3, 5), (4, 5), (5, 5), (6, 5) // Pentadecathlon
        ]),
        (23, 45, vec![
            (0, 2), (1, 2), (2, 2), (3, 2), //HWSS
            (0, 3), (3, 3),
            (1, 4), (2, 4), (3, 4)
        ]),
        (90, 16, vec![
            (1, 1), (1, 2), (1, 3),
            (2, 0), (2, 1), (2, 2) // Toad
        ]),
        (30, 65, vec![
            (1, 1), (1, 2), (1, 3),
            (2, 0), (2, 1), (2, 2) // Toad
        ]),
        (30, 50, vec![
            (1, 2), (2, 2), (3, 2), (4, 2), (5, 2), //Pulsar
            (1, 3), (5, 3), 
            (1, 4), (5, 4), 
            (1, 5), (2, 5), (3, 5), (4, 5), (5, 5), 
            (2, 1), (3, 1), (4, 1), 
            (2, 6), (3, 6), (4, 6),
            (1, 4), (1, 5),
            (5, 4), (5, 5)
        ]),
        (10, 70, vec![
            (1, 2), (2, 2), (3, 2), (4, 2), (5, 2), //Pulsar
            (1, 3), (5, 3), 
            (1, 4), (5, 4), 
            (1, 5), (2, 5), (3, 5), (4, 5), (5, 5), 
            (2, 1), (3, 1), (4, 1), 
            (2, 6), (3, 6), (4, 6),
            (1, 4), (1, 5),
            (5, 4), (5, 5)
        ])

    ];

    initialize_patterns(&mut framebuffer, patterns);

    let frame_delay = Duration::from_millis(200);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let new_buffer = next_generation(&framebuffer, framebuffer_width, framebuffer_height);
        framebuffer.buffer = new_buffer;

        window.update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}
