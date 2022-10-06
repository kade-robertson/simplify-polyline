#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

fn get_sq_dist(p1: &Point, p2: &Point) -> f64 {
    let dx = p2.x - p1.x;
    let dy = p2.y - p1.y;

    dx * dx + dy * dy
}

fn get_sq_seg_dist(pt: &Point, start: &Point, end: &Point) -> f64 {
    let (mut x, mut y, mut dx, mut dy) = (start.x, start.y, end.x - start.x, end.y - start.y);

    if dx != 0.0 || dy != 0.0 {
        let t = ((pt.x - x) * dx + (pt.y - y) * dy) / (dx * dx + dy * dy);

        if t > 1.0 {
            x = end.x;
            y = end.y;
        } else if t > 0.0 {
            x += dx * t;
            y += dy * t;
        }
    }

    dx = pt.x - x;
    dy = pt.y - y;

    dx * dx + dy * dy
}

fn simplify_radial_dist(points: &[Point], tolerance: f64) -> Vec<Point> {
    let mut prev_point = points[0];
    let mut new_points = vec![prev_point];
    let mut point = prev_point;

    for pt in points.iter().skip(1) {
        point = *pt;
        if get_sq_dist(pt, &prev_point) > tolerance {
            new_points.push(*pt);
            prev_point = *pt;
        }
    }

    if prev_point != point {
        new_points.push(point);
    }

    new_points
}

fn simplify_dp_step(
    points: &[Point],
    first: usize,
    last: usize,
    tolerance: f64,
    simplified: &mut Vec<Point>,
) {
    let mut max_sq_dist = tolerance;
    let mut max_index = 0;

    for i in first + 1..last {
        let sq_dist = get_sq_seg_dist(&points[i], &points[first], &points[last]);
        if sq_dist > max_sq_dist {
            max_index = i;
            max_sq_dist = sq_dist;
        }
    }

    if max_sq_dist > tolerance {
        if (max_index - first) > 1 {
            simplify_dp_step(points, first, max_index, tolerance, simplified);
        }
        simplified.push(points[max_index]);
        if (last - max_index) > 1 {
            simplify_dp_step(points, max_index, last, tolerance, simplified);
        }
    }
}

fn simplify_douglas_peucker(points: &[Point], tolerance: f64) -> Vec<Point> {
    let mut simplified = vec![points[0]];
    simplify_dp_step(points, 0, points.len() - 1, tolerance, &mut simplified);
    simplified.push(points[points.len() - 1]);

    simplified
}

pub fn simplify(points: &[Point], tolerance: f64, high_quality: bool) -> Vec<Point> {
    if points.len() <= 2 {
        return points.to_vec();
    }

    let tolerance_sq = tolerance * tolerance;
    let intermediate = if high_quality {
        points.to_vec()
    } else {
        simplify_radial_dist(points, tolerance_sq)
    };

    simplify_douglas_peucker(&intermediate, tolerance_sq)
}
