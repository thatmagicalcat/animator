use sfml::system::Clock;
use sfml::{graphics::*, system::*, window::*};

mod animator;

fn main() {
    let settings = ContextSettings {
        antialiasing_level: 16,
        ..Default::default()
    };

    let mut window = RenderWindow::new((800, 800), "testing", Style::CLOSE, &settings);
    window.set_framerate_limit(1000);

    let mut red_box = RectangleShape::new();
    red_box.set_size((200., 200.));
    red_box.set_fill_color(Color::RED);

    let mut green_box = RectangleShape::new();
    green_box.set_size((200., 200.));
    green_box.set_fill_color(Color::GREEN);

    let mut info_text = {
        let mut font: Box<sfml::SfBox<Font>> = Box::new(Font::from_file("Hack NF.ttf").unwrap());
        font.set_smooth(true);
        let mut text = Text::default();
        text.set_string("");
        text.set_font(Box::leak(font));
        text.set_character_size(30);
        text
    };

    info_text.set_position((10.0, 10.0));
    info_text.set_fill_color(Color::WHITE);

    let time = Time::seconds(10.0);

    let top_left: Vector2f = (0.0, 0.0).into();
    let top_right: Vector2f = (600.0, 0.0).into();
    let bottom_left: Vector2f = (0.0, 600.0).into();
    let bottom_right: Vector2f = (600.0, 600.0).into();

    let path = vec![
        bottom_left,
        bottom_right,
        top_right,
        bottom_left,
        top_right,
        top_left,
        bottom_right
    ];

    let mut animator_1 = animator::PathAnimation::new(
        top_left,
        top_left,
        time,
        path.clone(),
        ezing::cubic_inout,
    );

    let mut animator_2 = animator::PathAnimation::new(
        top_left,
        top_left,
        time,
        path.clone(),
        ezing::quad_inout,
    );

    let mut clock = Clock::start();
    while window.is_open() {
        let dt = clock.restart();

        animator_1.step(dt);
        animator_2.step(dt);

        red_box.set_position(animator_1.get_position());
        green_box.set_position(animator_2.get_position());

        if animator_1.finished() {
            animator_1.restart()
        };

        if animator_2.finished() {
            animator_2.restart()
        };

        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed => window.close(),
                _ => {}
            }
        }

        let fps = 1.0 / dt.as_seconds();
        info_text.set_string(&format!("FPS: {:.0}", fps,));

        window.clear(Color::BLACK);

        window.draw(&red_box);
        window.draw(&green_box);
        window.draw(&info_text);

        window.display()
    }
}
