use core::time;
use std::cmp;
use std::fs;

use nannou::prelude::*;

const RUNTIME_SECONDS: u64 = 60;

const OUTPUT_DIR: &'static str = "output/frames";

fn main() {
    fs::remove_dir_all(OUTPUT_DIR).expect("Could not remove output dir");

    nannou::app(model)
        .update(update)
        .simple_window(view)
        .size(1000, 1000)
        .run();
}

struct Model {
    runtime: Option<time::Duration>,

    // rotation of the object
    // [0, 2PI]
    rot: f32,

    // number of objects to draw (in a pattern)
    clones: i32,

    // spawn rate
    spawn_rate: i32,

    stripe_color: Hsl,
    stripe_spacing: f32,
}

fn model(app: &App) -> Model {
    app.set_loop_mode(LoopMode::rate_fps(60.));

    Model {
        runtime: Some(time::Duration::new(RUNTIME_SECONDS, 0)), // TODO: How do I get a CLI arg here?
        rot: 0.,
        clones: 0,
        spawn_rate: 1,
        stripe_color: hsl(40., 0.1, 0.1),
        stripe_spacing: 0.,
    }
}

fn update(app: &App, model: &mut Model, update: Update) {
    if let Some(runtime) = model.runtime {
        if update.since_start > runtime {
            return app.quit();
        }
    }

    let rate = app.time.sin();

    // spin the object
    model.rot += rate;

    // change the number of clones
    model.clones = cmp::max(0, cmp::min(100, model.clones + model.spawn_rate));

    // change spawn rate
    if model.clones == 100 {
        model.spawn_rate = -1;
    } else if model.clones == 0 {
        model.spawn_rate = 1;
    }

    model.stripe_color = hsl((app.time / 30.) % 360., 1., 0.2);
    model.stripe_spacing = triangle(10., 15., app.time);
}

fn triangle(amplitude: f32, period: f32, t: f32) -> f32 {
    amplitude * ((t + period / 2.) % period - period / 2.).abs()
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    // Stripes!
    for i in -5..6 {
        draw.rect()
            .color(model.stripe_color)
            .rotate(PI / 4.)
            .w_h(100., 5.)
            .x_y(
                (i as f32) * model.stripe_spacing,
                -(i as f32) * model.stripe_spacing,
            );

        draw.rect()
            .color(model.stripe_color)
            .rotate(-PI / 4.)
            .w_h(100., 5.)
            .x_y(
                (i as f32) * model.stripe_spacing,
                (i as f32) * model.stripe_spacing,
            );
    }

    let size = 20.;

    draw.rect().color(WHITE).rotate(model.rot).w_h(size, size);

    let a = 30.;
    let b = 20.;

    for i in 0..model.clones {
        let angle = 0.5 * (i as f32);
        let x = (a + b * angle) * angle.cos();
        let y = (a + b * angle) * angle.sin();
        draw.rect()
            .color(WHITE)
            .w_h(size, size)
            .x_y(x, y)
            .rotate(model.rot);
    }

    draw.to_frame(app, &frame).unwrap();

    // Capture the frame for stitching together later.
    let file_path = captured_frame_path(app, &frame);
    app.main_window().capture_frame(file_path);
}

fn captured_frame_path(app: &App, frame: &Frame) -> std::path::PathBuf {
    app.project_path()
        .expect("failed to locate `project_path`")
        .join(OUTPUT_DIR)
        // Use the frame number the a filename.
        .join(frame.nth().to_string())
        .with_extension("png")
}
