use nannou::prelude::*;
use std::cmp;

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Model {
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

fn model(_app: &App) -> Model {
    Model {
        rot: 0.,
        clones: 0,
        spawn_rate: 1,
        stripe_color: hsl(40., 0.1, 0.1),
        stripe_spacing: 0.,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
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
}
