use svg::Document;
use svg::node::element::Path;
use svg::node::element::path::Data;

use crate::lines::DirectedLineSegment;

pub fn render(lines: &[DirectedLineSegment], box_width: usize, box_height: usize) -> Document {
    let mut data = Data::new();

    for line in lines {
        let initial_point = line.initial_point;
        let terminal_point = line.terminal_point;

        data = data.move_to((initial_point.x, initial_point.y));
        data = data.line_to((terminal_point.x, terminal_point.y));
    }

    let path = Path::new()
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", 0.25)
        .set("d", data);

    Document::new()
        .set("viewBox", (0, 0, box_width, box_height))
        .add(path)
}
