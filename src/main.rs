use algorithm::{subdivide, sierpinski};
use lines::{DirectedLineSegment, Vector};
use renderer::render;


mod algorithm;
mod lines;
mod renderer;

fn main() {
    let my_segment = DirectedLineSegment::new(Vector::new(10.0, 110.0), Vector::new(110.0, 110.0));
    let lines = sierpinski(my_segment, 6);
    let document = render(&lines, 120, 120);

    svg::save("out/out.svg", &document).unwrap();
}
