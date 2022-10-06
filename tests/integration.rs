use simplify_polyline::{point, points, simplify, Point};

#[test]
fn returns_empty_vec_if_no_points() {
    let result = simplify(&[], 1.0, false);
    assert_eq!(result.len(), 0);
}

#[test]
fn returns_empty_vec_if_one_point() {
    let input = points!((0.0, 0.0));
    let result = simplify(input, 1.0, false);
    assert_eq!(result[0], input[0]);
}

#[test]
fn matches_expected_output() {
    let input = serde_json::from_str::<Vec<Point>>(include_str!("../fixtures/test-case.json"));
    let expected_output =
        serde_json::from_str::<Vec<Point>>(include_str!("../fixtures/test-case-output.json"));

    assert!(input.is_ok());
    assert!(expected_output.is_ok());

    let result = simplify(&input.unwrap(), 5.0, false);

    assert_eq!(result, expected_output.unwrap());
}
