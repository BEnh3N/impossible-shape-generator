use nannou::{prelude::*, winit::event::WindowEvent};
use nannou_egui::{Egui, egui::{self, Slider}};

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
    ui: Egui,
    sides: i32,
    size: f32,
    width: f32,
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .title("Impossible Shape Generator")
        .view(view)
        .raw_event(raw_event)
        .build()
        .unwrap();
    let ui = Egui::from_window(&app.window(_window).unwrap());

    let sides = 3;
    let size = 50.0;
    let width = 50.0;
    Model { _window, ui, sides, size, width }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    update_ui(model);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    draw.ellipse().x_y(0., 0.).w_h(5., 5.).color(BLUE);

    let sides = model.sides;
    let size = model.size;
    let width = model.width;
    let inner_angle = ((sides - 2) as f32 * 180.0).to_radians();
    let angle = inner_angle / sides as f32;

    let angle_offset = (2.0 * PI) / sides as f32;

    let mut points = [vec2(0., 0.); 7];

    let sin_length = width / angle.sin();
    let tan_length = width / angle.tan();

    let y_off = size / (2.0 * (PI / sides as f32).tan());
    points[0] = vec2(-(size / 2.0), -y_off);

    points[1] = vec2(points[0].x - tan_length, points[0].y - width);


    let x_off = size + 2.0 * sin_length + tan_length;
    points[2] = vec2(points[0].x + x_off, points[1].y);

    points[4] = rotate_around_origin(&points[2], -angle_offset);

    points[3] = rotate_around_origin(&vec2(points[1].x - sin_length * 2.0, points[1].y), -2.0 * angle_offset);

    points[5] = vec2(points[0].x + size + sin_length, points[0].y);

    points[6] = points[0];

    for i in 0..sides {
        for p in 0..points.len() {
            points[p] = rotate_around_origin(&points[p], angle_offset);
        }

        let c = (i+1) as f32 / sides as f32;
        let color = rgb(c, c, c);
        draw.polygon().points(points).color(color);
        draw.polyline().points(points).color(WHITE);
    }
    
    draw.to_frame(app, &frame).unwrap();
    model.ui.draw_to_frame(&frame).unwrap();
}

fn rotate_around_origin(point: &Vec2, angle: f32) -> Vec2 {
    let angle = -angle;
    let x = point.x * angle.cos() - point.y * angle.sin();
    let y = point.y * angle.cos() + point.x * angle.sin();
    vec2(x, y)
}

fn raw_event(_app: &App, model: &mut Model, event: &WindowEvent) {
    model.ui.handle_raw_event(event);
}

fn update_ui(model: &mut Model) {
    let ctx = model.ui.begin_frame();
    egui::Window::new("Options")
        .collapsible(true)
        .show(&ctx, |ui| {
            ui.label("Sides");
            ui.add(Slider::new(&mut model.sides, 3..=10));
            ui.label("Inner Size");
            ui.add(Slider::new(&mut model.size, 0.0..=250.0));
            ui.label("Width");
            ui.add(Slider::new(&mut model.width, 0.0..=250.0));
        });
}
