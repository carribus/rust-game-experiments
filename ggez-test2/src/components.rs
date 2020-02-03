use ggez::graphics::Color;

#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct Velocity {
    pub xv: f32,
    pub yv: f32,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ShapeType {
    Rectangle(f32, f32),
    Circle(f32),
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Shape {
    pub shape_type: ShapeType,
    pub colour: Color,
}
