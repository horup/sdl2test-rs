use std::{env, time::Instant};
use sdl2::{image::{LoadTexture, InitFlag}, rect::Rect};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
mod backend;
use backend as b;
mod backends;
use backends::SDL2Backend;

fn main()
{
    let mut backend = SDL2Backend::new();

    let sheet_img = include_bytes!("sheet.png");
    let sheet_texture = backend.load_texture(sheet_img, 16.0, 16.0, 0);

    let mut now = Instant::now();
    let mut frames = 0;

    while backend.keep_running()
    {
        backend.poll_events();
        let l = 1024;
        let size = 1.0;
        for y in 0..l
        {
            for x in 0..l
            {
                let i = x % 2;
                let dist = b::Rect {
                    x:x as f32 * size,
                    y:y as f32 * size,
                    w:size,
                    h:size
                };

                let src = b::Cell {col:i as f32, row:0.0};
                backend.draw_sprite(&dist, &src, 0);

            }
        }

        backend.present();
        frames += 1;

        if now.elapsed().as_millis() > 1000
        {
            let avg = now.elapsed().as_millis() as f32 / frames as f32;
            println!("{} fps ({}ms) to copy {} textures", frames, avg, l*l);
            frames = 0;
            now = Instant::now();
        }
    }
}