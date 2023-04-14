use std::f64::consts::PI;

use crate::lines::DirectedLineSegment;

const SEGMENT_0_ANGlE: f64 = -60.0 / 180.0 * PI;
const SEGMENT_2_ANGlE: f64 = 60.0 / 180.0 * PI;

pub fn subdivide(line: DirectedLineSegment) -> Vec<DirectedLineSegment> {
    let initial_point = line.initial_point;
    let relative_vector = line.relative_vector();
    let slice = relative_vector / 2.0;

    let segment_0 = DirectedLineSegment::new(initial_point, initial_point + slice.rotate(SEGMENT_0_ANGlE));
    let segment_1 = DirectedLineSegment::new(segment_0.terminal_point, segment_0.terminal_point + slice);
    let segment_2 = DirectedLineSegment::new(segment_1.terminal_point, segment_1.terminal_point + slice.rotate(SEGMENT_2_ANGlE));

    // the reverse is a hack so that it alternates between convex and concave.
    vec![segment_0.reverse(), segment_1, segment_2.reverse()]
}

pub fn sierpinski(line: DirectedLineSegment, iterations: usize) -> Vec<DirectedLineSegment> {
    let mut lines = vec![line];
    for _ in 0..iterations {
        let mut new_lines = Vec::new();
        for line in lines {
            let sub_lines = subdivide(line);
            new_lines.extend(sub_lines);
        }
        lines = new_lines;
    }
    lines
}
