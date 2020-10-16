use std::collections::HashMap;

use sdl2::{Sdl, VideoSubsystem, event::Event, image::LoadTexture, keyboard::Keycode, rect::Rect, render::Canvas, render::TextureCreator, video::Window, video::WindowContext};
use crate::backend as b;
pub struct SDL2Backend
{
    pub sdl_context:Sdl,
    pub video_subsystem:VideoSubsystem,
    pub keep_running:bool,
    pub canvas:Canvas<Window>,
    pub textures:HashMap<i32, sdl2::render::Texture>
}

impl crate::backend::Backend for SDL2Backend
{
    fn poll_events(&mut self) 
    {
        for event in self.sdl_context.event_pump().expect("event_pump failed in sdl2").poll_iter() {
            match event {
                Event::Quit{..} |
                Event::KeyDown {keycode: Option::Some(Keycode::Escape), ..} => self.keep_running = false,
                _ => {}
            }
        }
    }

    fn keep_running(&mut self) -> bool {
        self.keep_running
    }

    fn present(&mut self) {
        self.canvas.present();
    }

    fn create_texture(&mut self, texture_data:&[u8], cols:f32, rows:f32) -> crate::backend::Texture {
        let texture_creator = self.canvas.texture_creator();
        let sdl_texture = texture_creator.load_texture_bytes(texture_data).expect("loading of texture failed");
        let texture = crate::backend::Texture {
            id:self.textures.len() as i32,
            width:sdl_texture.query().width as f32,
            height:sdl_texture.query().height as f32,
            cols,
            rows
        };

        self.textures.insert(texture.id, sdl_texture);

        return  texture;
    }

    //fn draw_sprite(&mut self, x:f32, y:f32, col:f32, row:f32, tex:&crate::backend::Texture) {
    fn draw_sprite(&mut self, dist:&b::Rect, src:&b::Cell, tex:&b::Texture)
    {
        let w = tex.width / tex.cols;
        let h = tex.height / tex.rows;

        let src = Rect::new((src.col as f32 / tex.cols) as i32, (src.row / tex.rows) as i32, w as u32,h as u32);
        let dist = Rect::new(dist.x as i32, dist.y as i32, dist.w as u32, dist.h as u32);
        if let Some(texture) = self.textures.get(&tex.id)
        {
            self.canvas.copy(&texture, Some(src), Some(dist)).expect("failed to copy");
        }
    }
}

impl SDL2Backend
{
    pub fn new() -> Box<dyn crate::backend::Backend> 
    {
        let sdl_context = sdl2::init().expect("could not init sdl");
        let video_subsystem = sdl_context.video().expect("could not init video subsystem");
        let window = video_subsystem.window("rust-sdl2 demo: Video", 1920, 1080)
        .position_centered()
        .build()
        .map_err(|e| e.to_string()).expect("could not create window");
        let canvas = window.into_canvas().build().map_err(|e| e.to_string()).expect("could not convert window into canvas");
        Box::new(SDL2Backend {
            sdl_context,
            video_subsystem,
            keep_running:true,
            canvas,
            textures:HashMap::new()
        })
    }
}