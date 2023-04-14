use lines::{DirectedLineSegment, Vector};
use renderer::render;



mod lines;
mod renderer;

fn main() {
    let lines = vec![
        DirectedLineSegment::new(Vector::new(10.0, 110.0), Vector::new(110.0, 110.0)),
    ];
    let document = render(&lines, 120, 120);

    svg::save("out/out.svg", &document).unwrap();
}
