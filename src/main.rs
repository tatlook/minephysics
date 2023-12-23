mod object;
mod render;

use std::time::Instant;

use object::Object;
use sfml::{
    graphics::{Color, RenderTarget, RenderWindow},
    window::{Style, VideoMode},
};

fn get_objects() -> Vec<Object> {
    vec![
        Object::new((1000.0, 500.0), (0.0, 0.0), 2000.0),
        Object::new((400.0, 500.0), (0.0, 3.0), 80.0),
        Object::new((200.0, 500.0), (0.0, 4.0), 80.0),
        Object::new((1800.0, 500.0), (0.0, -4.0), 80.0),
        Object::new((1600.0, 500.0), (0.0, -3.0), 80.0),
        Object::new((600.0, 500.0), (0.0, 4.0), 10.0),
        Object::new((800.0, 500.0), (0.0, 4.0), 10.0),
        Object::new((850.0, 500.0), (0.0, 4.0), 10.0),
        Object::new((1300.0, 500.0), (0.0, -4.0), 10.0),
        Object::new((1400.0, 500.0), (0.0, -8.0), 10.0),
    ]
}

fn main() {
    let play_speed = 10.0;

    let mut window = RenderWindow::new(
        VideoMode::desktop_mode(),
        "Minephysics",
        Style::FULLSCREEN,
        &Default::default(),
    );
    let mut objects = get_objects();
    let mut shapes = render::create_shapes_from_objects(&objects);
    while window.is_open() {
        let now = Instant::now();
        while let Some(event) = window.poll_event() {
            match event {
                sfml::window::Event::Closed => window.close(),
                _ => {}
            }
        }

        render::set_shape_state(&objects, &mut shapes);
        render::draw_shapes(&mut window, &shapes);

        window.display();
        window.clear(Color::BLACK);

        object::perform(&mut objects, now.elapsed().as_secs_f32() * play_speed);
    }
}
