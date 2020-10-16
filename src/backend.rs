//pub const MAX_TEXTURES:u16 = 16;
pub trait Backend
{
    fn keep_running(&mut self) -> bool;
    fn poll_events(&mut self);
    fn present(&mut self);

    
    fn load_texture(&mut self, texture_data:&[u8], cols:f32, rows:f32, index:usize) -> Texture;
    //fn draw_sprite(&mut self, x:f32, y:f32, col:f32, row:f32, tex:&Texture);
    fn draw_sprite(&mut self, dist:&Rect, src:&Cell, index:usize);
}

#[derive(Debug, Copy, Clone)]
pub struct Texture
{
    pub id:usize,
    pub width:f32,
    pub height:f32,
    pub cols:f32,
    pub rows:f32
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Rect
{
    pub x:f32,
    pub y:f32,
    pub w:f32,
    pub h:f32
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Cell
{
    pub col:f32,
    pub row:f32
}



