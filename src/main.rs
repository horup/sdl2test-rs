#[link(name = "vcruntime", kind = "static")]

use std::{env, time::Instant};
use sdl2::{image::{LoadTexture, InitFlag}, rect::Rect};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub fn run() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;
    let window = video_subsystem.window("rust-sdl2 demo: Video", 1920, 1080)
      .position_centered()
      .build()
      .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().software().build().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();
    let sheet_img = include_bytes!("sheet.png");
    let texture = texture_creator.load_texture_bytes(sheet_img)?;
    let q = texture.query();
    let sheet_w = q.width;
    let sheet_h = q.height;
    let sheet_col = sheet_w / 16;
    let sheet_rows = sheet_h / 16;
    let mut now = Instant::now();
    let mut frames = 0;

    'mainloop: loop {
        for event in sdl_context.event_pump()?.poll_iter() {
            match event {
                Event::Quit{..} |
                Event::KeyDown {keycode: Option::Some(Keycode::Escape), ..} =>
                    break 'mainloop,
                _ => {}
            }
        }

        let l = 1024;
        let size = 32;
        for y in 0..l
        {
            for x in 0..l
            {
                let i = x % 2;
                let src = Rect::new(i*16,0, 16,16);
                let dist = Rect::new(x*size, y*size, size as u32, size as u32);
                canvas.copy(&texture, Some(src), Some(dist))?;
            }
        }


        canvas.present();
        frames += 1;

        if now.elapsed().as_millis() > 1000
        {
            let avg = now.elapsed().as_millis() as f32 / frames as f32;
            println!("{} fps ({}ms) to copy {} textures", frames, avg, l*l);
            frames = 0;
            now = Instant::now();
        }
    }

    Ok(())
}

fn main() -> Result<(), String> {
    let args: Vec<_> = env::args().collect();

    run()?;

    Ok(())
}