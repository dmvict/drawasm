
#[derive(Clone, Debug)]
pub enum Shape
{
  Rect( Rect ),
  FreeLine,
  Eraser,
}

#[derive(Clone, Debug)]
pub struct Item
{
  id : usize,
  shape : Shape,
}

impl Item
{
  pub fn new( shape : Shape ) -> Item
  {
    Item { id : 0, shape }
  }
}

#[derive(Clone, Debug)]
pub struct Rect
{
  x : f64,
  y : f64,
  width : f64,
  height : f64,
  angle : f64,
}

impl Rect
{
  pub fn new( x : f64, y : f64, width : f64, height : f64, angle : f64 ) -> Rect
  {
    Rect { x, y, width, height, angle }
  }
}

//

pub struct State
{
  width: u32,
  height: u32,
  preview_w: u32,
  preview_h: u32,
  pen_thin: f64,
  start_x : f64,
  start_y : f64,
  color: String,
  preview_image: Vec<String>,
  undo_image_data: Vec<web_sys::ImageData>,
  redo_image_data: Vec<web_sys::ImageData>,
  frame_speed: f64,
  action : Shape,
  shapes : Vec<Item>,
}

impl State {
    pub fn new(w: u32, h: u32) -> State {
        State {
            width: w,
            height: h,
            preview_w: w / 5,
            preview_h: h / 5,
            pen_thin: 1.0,                //TODO not hardcode
            start_x: 0.0,                //TODO not hardcode
            start_y: 0.0,                //TODO not hardcode
            color: "#000000".to_string(), //TODO not hardcode
            preview_image: vec![],
            undo_image_data: vec![],
            redo_image_data: vec![],
            frame_speed: 0.33,
            action : Shape::FreeLine,
            shapes : vec![],
        }
    }

    pub fn get_action(&self) -> Shape {
        self.action.clone()
    }

    pub fn set_action(&mut self, action: Shape) {
        self.action = action;
    }

    pub fn get_color(&self) -> String {
        self.color.clone() // not implement Copy trait
    }

    pub fn set_color(&mut self, color: String) {
        self.color = color;
    }

    pub fn get_pen_thin(&self) -> f64 {
        self.pen_thin
    }

    pub fn set_pen_thin(&mut self, pen_thin: f64) {
        self.pen_thin = pen_thin;
    }

    pub fn get_start_x(&self) -> f64 {
        self.start_x
    }

    pub fn set_start_x(&mut self, start_x: f64) {
        self.start_x = start_x;
    }

    pub fn get_start_y(&self) -> f64 {
        self.start_y
    }

    pub fn set_start_y(&mut self, start_y: f64) {
        self.start_y = start_y;
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn get_preview_width(&self) -> u32 {
        self.preview_w
    }

    pub fn get_preview_height(&self) -> u32 {
        self.preview_h
    }

    pub fn add_undo(&mut self, data: web_sys::ImageData) {
        self.undo_image_data.push(data);
    }

    pub fn get_undo(&mut self) -> Option<web_sys::ImageData> {
        self.undo_image_data.pop()
    }

    pub fn add_redo(&mut self, data: web_sys::ImageData) {
        self.redo_image_data.push(data);
    }

    pub fn get_redo(&mut self) -> Option<web_sys::ImageData> {
        self.redo_image_data.pop()
    }

    pub fn add_preview_image(&mut self, data: String) {
        self.preview_image.push(data);
    }

    pub fn get_preview_image(&self) -> Vec<String> {
        self.preview_image.clone()
    }

    pub fn get_preview_image_len(&self) -> usize {
        self.preview_image.len()
    }

    // TODO
    //pub fn delete_image(&mut self) -> Result<()> {

    //}

    pub fn delete_all_images(&mut self) {
        self.preview_image = vec![];
    }

    pub fn get_frame_speed(&self) -> f64 {
        self.frame_speed
    }

    pub fn set_frame_speed(&mut self, frame_speed: f64) {
        self.frame_speed = frame_speed;
    }

    pub fn item_push(&mut self, item: Item) {
        self.shapes.push( item );
    }
}
