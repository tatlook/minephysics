use sfml::graphics::{CircleShape, Color, RenderTarget, Shape, Transformable};

use crate::object::Object;

fn color_list() -> Vec<Color> {
    vec![
        Color::RED,
        Color::BLUE,
        Color::GREEN,
        Color::YELLOW,
        Color::CYAN,
        Color::MAGENTA,
    ]
}

pub fn create_shapes_from_objects<'a>(objects: &Vec<Object>) -> Vec<CircleShape<'a>> {
    let mut vec = vec![];
    for i in 0..objects.len() {
        let o = &objects[i];
        let mut shape = CircleShape::new(o.radius(), 10);
        shape.set_fill_color(color_list()[i % 6]);
        shape.set_outline_color(color_list()[i % 5 + 1]);
        shape.set_outline_thickness(3.0);
        vec.push(shape);
    }
    vec
}

pub fn set_shape_state<'a>(objects: &Vec<Object>, shapes: &mut Vec<CircleShape<'a>>) {
    assert_eq!(objects.len(), shapes.len());
    let mut i = 0;
    while i < objects.len() {
        let object = &objects[i];
        let shape = &mut shapes[i];
        let mut pos = object.position();
        pos.x -= object.radius(); // Make it to central
        pos.y -= object.radius();
        shape.set_position(pos);
        i += 1;
    }
}

pub fn draw_shapes<'a>(target: &mut dyn RenderTarget, shapes: &Vec<CircleShape<'a>>) {
    for shape in shapes {
        target.draw(shape);
    }
}
