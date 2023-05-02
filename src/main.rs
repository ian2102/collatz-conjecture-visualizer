use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};

static mut NUM: i64 = 0;

fn main() {
    nannou::app(model).update(update).run();
}

struct Settings {
    rotation: f32,
    color: Hsv,
    start_num: i64,
    line_length: f32,
    line_width: f32,
    random_values: bool,
    reps: i64,
    is_running: bool,
}

struct Model {
    settings: Settings,
    egui: Egui,
}

fn model(app: &App) -> Model {
    // Create window
    let window_id = app
        .new_window()
        .size(800, 600)
        .title("Collatz Conjecture Visualizer")
        .view(view)
        .raw_event(raw_window_event)
        .build()
        .unwrap();
    let window = app.window(window_id).unwrap();

    let egui = Egui::from_window(&window);

    Model {
        egui,
        settings: Settings {
            rotation: 10.0,
            color: hsv(10.0, 1.0, 0.5),
            start_num: 10,
            line_length: 20.0,
            line_width: 1.0,
            random_values: true,
            reps: 10,
            is_running: false,
        },
    }
}

fn update(_app: &App, model: &mut Model, update: Update) {
    let egui = &mut model.egui;
    let settings = &mut model.settings;

    egui.set_elapsed_time(update.since_start);
    let ctx = egui.begin_frame();
    
    egui::Window::new("Settings").show(&ctx, |ui| {

        ui.label("Rotation:");
        ui.add(egui::Slider::new(&mut settings.rotation, 0.0..=360.0));


        ui.label("Color Picker:");
        edit_hsv(ui, &mut settings.color);

        ui.label("Starting number:");
        ui.add(egui::Slider::new(&mut settings.start_num, 1..=10000000));

        ui.label("Line length:");
        ui.add(egui::Slider::new(&mut settings.line_length, 1.0..=100.0));

        ui.label("Line width:");
        ui.add(egui::Slider::new(&mut settings.line_width, 1.0..=10.0));

        ui.checkbox(&mut settings.random_values, "Random values");

        ui.label("Reps:");
        ui.add(egui::Slider::new(&mut settings.reps, 1..=100));

        let text: &str;
        if settings.is_running {
            text = "Stop";
        } else {
            text = "Start";
        }

        let is_running = ui.button(text).clicked();

        ui.label(format!("Calculations Completed: {}", unsafe{&NUM.to_string()}));

        if is_running {
            settings.is_running = !settings.is_running;
        }
        
    });
}


fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.egui.handle_raw_event(event);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let settings = &model.settings;
    let window = app.main_window();
    let win = window.rect();
    let draw = app.draw();
    draw.background().color(BLACK);

    if settings.is_running {
        draw_collatz(&draw, settings, &win);
    }

    draw.to_frame(app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();
}

fn draw_collatz(draw: &Draw, settings: &Settings, win: &Rect) {
    let mut value_list: Vec<i64> = vec![];
    for i in settings.start_num..(settings.start_num + settings.reps) {
        if settings.random_values {
            value_list.push(random_range(settings.start_num, settings.start_num * 100));
        } else {
            value_list.push(i);
        }
    }

    let angle = 90.0_f32.to_radians();
    let start_pos =  pt2(0.0, win.h() / -2.0);
    for mut n in value_list {
        let mut start_pos = start_pos;
        let mut angle = angle;
        while n > 1 {
            unsafe{NUM += 1;}
            if n % 2 == 0 {
                n /= 2;
                angle -= settings.rotation.to_radians();
            } else {
                n = (3 * n + 1) / 2;
                angle += settings.rotation.to_radians();
            }

            let end_pos = pt2(
                start_pos.x + settings.line_length * angle.cos(),
                start_pos.y + settings.line_length * angle.sin(),
            );

            draw.line()
                .start(start_pos)
                .end(end_pos)
                .color(settings.color)
                .stroke_weight(settings.line_width);

            start_pos = end_pos;
        }
    }
}


fn edit_hsv(ui: &mut egui::Ui, color: &mut Hsv) {
    let mut egui_hsv = egui::color::Hsva::new(
        color.hue.to_positive_radians() as f32 / (std::f32::consts::PI * 2.0),
        color.saturation,
        color.value,
        1.0,
    );

    if egui::color_picker::color_edit_button_hsva(
        ui,
        &mut egui_hsv,
        egui::color_picker::Alpha::Opaque,
    )
    .changed()
    {
        *color = nannou::color::hsv(egui_hsv.h, egui_hsv.s, egui_hsv.v);
    }
}