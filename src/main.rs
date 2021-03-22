use core::time;
use std::cmp;
use std::fs;
use std::path;

use nannou::prelude::*;

use humantime::parse_duration;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opts {
    #[structopt(long, parse(try_from_str = parse_duration))]
    duration: Option<time::Duration>,

    #[structopt(long, parse(from_os_str))]
    output_dir: Option<path::PathBuf>,
}

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .size(1000, 1000)
        .run();
}

struct Model {
    duration: Option<time::Duration>,
    output_dir: Option<path::PathBuf>,

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
    let opts = Opts::from_args();

    if let Some(dir) = &opts.output_dir {
        fs::remove_dir_all(dir).unwrap_or_else(|_| println!("Could not remove output dir"));
        fs::create_dir_all(dir).expect("Could not create output dir");
    }

    app.set_loop_mode(LoopMode::rate_fps(60.));

    Model {
        duration: opts.duration,
        output_dir: opts.output_dir,
        rot: 0.,
        clones: 0,
        spawn_rate: 1,
        stripe_color: hsl(40., 0.1, 0.1),
        stripe_spacing: 0.,
    }
}

fn update(app: &App, model: &mut Model, update: Update) {
    if let Some(duration) = model.duration {
        if update.since_start > duration {
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

    if let Some(dir) = &model.output_dir {
        // Capture the frame for stitching together later.
        let file_path = captured_frame_path(dir, &frame);
        app.main_window().capture_frame(file_path);
    }
}

fn captured_frame_path(dir: &path::PathBuf, frame: &Frame) -> std::path::PathBuf {
    dir.join("frames")
        .join(frame.nth().to_string())
        .with_extension("png")
}
