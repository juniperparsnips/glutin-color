use glutin_window::GlutinWindow;
use graphics::Transformed;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::{EventSettings, Events, RenderArgs, RenderEvent, WindowSettings};

const WHITE: [f32; 4] = [1.0; 4];
static SIZE: [f64; 2] = [400.0, 400.0];

fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: GlutinWindow = WindowSettings::new("hello color!", SIZE)
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();
    let mut gl = GlGraphics::new(opengl);

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            render_boxes(&mut gl, &args);
        }
    }
}

fn render_boxes(gl: &mut GlGraphics, args: &RenderArgs) {
    gl.draw(args.viewport(), |ctx, gl| {
        graphics::clear(WHITE, gl);

        let n = 8;

        let w = SIZE[0] / n as f64;
        let rect = graphics::rectangle::rectangle_by_corners(0.0, 0.0, w, w);

        let colors = greyscales(n * n);

        for i in 0..n * n {
            graphics::rectangle(
                colors[i as usize],
                rect,
                ctx.transform
                    .trans(w * (i % n) as f64, w * f64::floor(i as f64 / n as f64)),
                gl,
            );
        }
    });
}

fn greyscales(number: u8) -> Vec<[f32; 4]> {
    let mut colors = Vec::with_capacity(number.into());

    let gap = 1.0 / number as f32;

    for i in 0..number {
        let x = i as f32 * gap;
        colors.push([x, x, x, 1.0]);
    }

    colors
}
