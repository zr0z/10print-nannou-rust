use nannou::prelude::*;
use nannou::rand::{thread_rng, Rng};

const WIDTH: u32 = 512;
const OFFSET: f32 = 8.0;
const DOUBLE_OFFSET: f32 = OFFSET * 2.0;
const TOP_LEFT: f32 = (WIDTH / 2) as f32 - OFFSET;
const LINE_WEIGHT: f32 = 2.0;

fn main() {
    nannou::sketch(view)
        .size(WIDTH, WIDTH)
        .loop_mode(LoopMode::loop_once())
        .run();
}

fn view(app: &App, frame: Frame) {
    // get canvas to draw on
    let draw = app.draw();

    // Generate a random integer
    let mut rng = thread_rng();
    let mut angle = || rng.gen_range(0..2) as f32;

    // set background to blue
    draw.background().color(rgb(0u8, 130u8, 200u8));

    // Create start and end points of the line
    let start_point = pt2(-OFFSET, -OFFSET);
    let end_point = pt2(OFFSET, OFFSET);

    // Create a drawing function positioned on the top left of the window
    let ldraw = draw.x_y(-TOP_LEFT, -TOP_LEFT);

    // Declare the coordinates where the lines are going to be drawn
    let mut x = 0.0;
    let mut y = 0.0;

    // Loop until y is larger than the window's height
    while y < WIDTH as f32 {
        // If x is larger than the width of the window we reset x and increment y
        if x > WIDTH as f32 {
            x = 0.0;
            y += DOUBLE_OFFSET;
        }
        // Randomly set the line angle to 0 or 90
        ldraw
            .line()
            .start(start_point)
            .end(end_point)
            .weight(LINE_WEIGHT)
            .z_degrees(angle() * 90.0)
            .x_y(x, y)
            .color(WHITE);

        // Increment x
        x += DOUBLE_OFFSET;
    }

    // put everything on the frame
    draw.to_frame(app, &frame).unwrap();
}
