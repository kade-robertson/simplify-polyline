use simplify_polyline::{point, points, simplify, Point};

#[test]
fn returns_empty_vec_if_no_points() {
    let result = simplify::<f64>(&[], 1.0, false);
    assert_eq!(result.len(), 0);
}

#[test]
fn returns_same_vec_if_one_point() {
    let input1 = points![(0.0, 0.0)];
    let result1 = simplify(input1, 1.0, false);
    assert_eq!(result1[0], input1[0]);

    let input2 = point!(0.0, 0.0);
    let result2 = simplify(&[input2], 1.0, false);
    assert_eq!(result1, result2);
}

#[test]
fn matches_expected_output() {
    let input = serde_json::from_str::<Vec<Point<f64>>>(include_str!("../fixtures/test-case.json"));
    let expected_output =
        serde_json::from_str::<Vec<Point<f64>>>(include_str!("../fixtures/test-case-output.json"));

    assert!(input.is_ok());
    assert!(expected_output.is_ok());

    let result = simplify(&input.unwrap(), 5.0, false);

    assert_eq!(result, expected_output.unwrap());
}
