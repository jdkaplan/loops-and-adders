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
}

fn model(_app: &App) -> Model {
    Model {
        rot: 0.,
        clones: 0,
        spawn_rate: 1,
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
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

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
