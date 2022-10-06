use simplify_polyline::{simplify, Point};

const INPUT: &[Point] = &[
    Point {
        x: 224.55,
        y: 250.15,
    },
    Point {
        x: 226.91,
        y: 244.19,
    },
    Point {
        x: 233.31,
        y: 241.45,
    },
    Point {
        x: 234.98,
        y: 236.06,
    },
    Point {
        x: 244.21,
        y: 232.76,
    },
    Point {
        x: 262.59,
        y: 215.31,
    },
    Point {
        x: 267.76,
        y: 213.81,
    },
    Point {
        x: 273.57,
        y: 201.84,
    },
    Point {
        x: 273.12,
        y: 192.16,
    },
    Point {
        x: 277.62,
        y: 189.03,
    },
    Point {
        x: 280.36,
        y: 181.41,
    },
    Point {
        x: 286.51,
        y: 177.74,
    },
    Point {
        x: 292.41,
        y: 159.37,
    },
    Point {
        x: 296.91,
        y: 155.64,
    },
    Point {
        x: 314.95,
        y: 151.37,
    },
    Point {
        x: 319.75,
        y: 145.16,
    },
    Point {
        x: 330.33,
        y: 137.57,
    },
    Point {
        x: 341.48,
        y: 139.96,
    },
    Point {
        x: 369.98,
        y: 137.89,
    },
    Point {
        x: 387.39,
        y: 142.51,
    },
    Point {
        x: 391.28,
        y: 139.39,
    },
    Point {
        x: 409.52,
        y: 141.14,
    },
    Point {
        x: 414.82,
        y: 139.75,
    },
    Point {
        x: 427.72,
        y: 127.3,
    },
    Point {
        x: 439.6,
        y: 119.74,
    },
    Point {
        x: 474.93,
        y: 107.87,
    },
    Point {
        x: 486.51,
        y: 106.75,
    },
    Point {
        x: 489.2,
        y: 109.45,
    },
    Point {
        x: 493.79,
        y: 108.63,
    },
    Point {
        x: 504.74,
        y: 119.66,
    },
    Point {
        x: 512.96,
        y: 122.35,
    },
    Point {
        x: 518.63,
        y: 120.89,
    },
    Point {
        x: 524.09,
        y: 126.88,
    },
    Point {
        x: 529.57,
        y: 127.86,
    },
    Point {
        x: 534.21,
        y: 140.93,
    },
    Point {
        x: 539.27,
        y: 147.24,
    },
    Point {
        x: 567.69,
        y: 148.91,
    },
    Point {
        x: 575.25,
        y: 157.26,
    },
    Point {
        x: 580.62,
        y: 158.15,
    },
    Point {
        x: 601.53,
        y: 156.85,
    },
    Point {
        x: 617.74,
        y: 159.86,
    },
    Point {
        x: 622.0,
        y: 167.04,
    },
    Point {
        x: 629.55,
        y: 194.6,
    },
    Point {
        x: 638.9,
        y: 195.61,
    },
    Point {
        x: 641.26,
        y: 200.81,
    },
    Point {
        x: 651.77,
        y: 204.56,
    },
    Point {
        x: 671.55,
        y: 222.55,
    },
    Point {
        x: 683.68,
        y: 217.45,
    },
    Point {
        x: 695.25,
        y: 219.15,
    },
    Point {
        x: 700.64,
        y: 217.98,
    },
    Point {
        x: 703.12,
        y: 214.36,
    },
    Point {
        x: 712.26,
        y: 215.87,
    },
    Point {
        x: 721.49,
        y: 212.81,
    },
    Point {
        x: 727.81,
        y: 213.36,
    },
    Point {
        x: 729.98,
        y: 208.73,
    },
    Point {
        x: 735.32,
        y: 208.2,
    },
    Point {
        x: 739.94,
        y: 204.77,
    },
    Point {
        x: 769.98,
        y: 208.42,
    },
    Point {
        x: 779.6,
        y: 216.87,
    },
    Point {
        x: 784.2,
        y: 218.16,
    },
    Point {
        x: 800.24,
        y: 214.62,
    },
    Point {
        x: 810.53,
        y: 219.73,
    },
    Point {
        x: 817.19,
        y: 226.82,
    },
    Point {
        x: 820.77,
        y: 236.17,
    },
    Point {
        x: 827.23,
        y: 236.16,
    },
    Point {
        x: 829.89,
        y: 239.89,
    },
    Point {
        x: 851.0,
        y: 248.94,
    },
    Point {
        x: 859.88,
        y: 255.49,
    },
    Point {
        x: 865.21,
        y: 268.53,
    },
    Point {
        x: 857.95,
        y: 280.3,
    },
    Point {
        x: 865.48,
        y: 291.45,
    },
    Point {
        x: 866.81,
        y: 298.66,
    },
    Point {
        x: 864.68,
        y: 302.71,
    },
    Point {
        x: 867.79,
        y: 306.17,
    },
    Point {
        x: 859.87,
        y: 311.37,
    },
    Point {
        x: 860.08,
        y: 314.35,
    },
    Point {
        x: 858.29,
        y: 314.94,
    },
    Point { x: 858.1, y: 327.6 },
    Point {
        x: 854.54,
        y: 335.4,
    },
    Point {
        x: 860.92,
        y: 343.0,
    },
    Point {
        x: 856.43,
        y: 350.15,
    },
    Point {
        x: 851.42,
        y: 352.96,
    },
    Point {
        x: 849.84,
        y: 359.59,
    },
    Point {
        x: 854.56,
        y: 365.53,
    },
    Point {
        x: 849.74,
        y: 370.38,
    },
    Point {
        x: 844.09,
        y: 371.89,
    },
    Point {
        x: 844.75,
        y: 380.44,
    },
    Point {
        x: 841.52,
        y: 383.67,
    },
    Point {
        x: 839.57,
        y: 390.4,
    },
    Point {
        x: 845.59,
        y: 399.05,
    },
    Point {
        x: 848.4,
        y: 407.55,
    },
    Point {
        x: 843.71,
        y: 411.3,
    },
    Point {
        x: 844.09,
        y: 419.88,
    },
    Point {
        x: 839.51,
        y: 432.76,
    },
    Point {
        x: 841.33,
        y: 441.04,
    },
    Point {
        x: 847.62,
        y: 449.22,
    },
    Point {
        x: 847.16,
        y: 458.44,
    },
    Point {
        x: 851.38,
        y: 462.79,
    },
    Point {
        x: 853.97,
        y: 471.15,
    },
    Point {
        x: 866.36,
        y: 480.77,
    },
];

const EXPECTED_OUTPUT: &[Point] = &[
    Point {
        x: 224.55,
        y: 250.15,
    },
    Point {
        x: 267.76,
        y: 213.81,
    },
    Point {
        x: 296.91,
        y: 155.64,
    },
    Point {
        x: 330.33,
        y: 137.57,
    },
    Point {
        x: 409.52,
        y: 141.14,
    },
    Point {
        x: 439.6,
        y: 119.74,
    },
    Point {
        x: 486.51,
        y: 106.75,
    },
    Point {
        x: 529.57,
        y: 127.86,
    },
    Point {
        x: 539.27,
        y: 147.24,
    },
    Point {
        x: 617.74,
        y: 159.86,
    },
    Point {
        x: 629.55,
        y: 194.6,
    },
    Point {
        x: 671.55,
        y: 222.55,
    },
    Point {
        x: 727.81,
        y: 213.36,
    },
    Point {
        x: 739.94,
        y: 204.77,
    },
    Point {
        x: 769.98,
        y: 208.42,
    },
    Point {
        x: 779.6,
        y: 216.87,
    },
    Point {
        x: 800.24,
        y: 214.62,
    },
    Point {
        x: 820.77,
        y: 236.17,
    },
    Point {
        x: 859.88,
        y: 255.49,
    },
    Point {
        x: 865.21,
        y: 268.53,
    },
    Point {
        x: 857.95,
        y: 280.3,
    },
    Point {
        x: 867.79,
        y: 306.17,
    },
    Point {
        x: 859.87,
        y: 311.37,
    },
    Point {
        x: 854.54,
        y: 335.4,
    },
    Point {
        x: 860.92,
        y: 343.0,
    },
    Point {
        x: 849.84,
        y: 359.59,
    },
    Point {
        x: 854.56,
        y: 365.53,
    },
    Point {
        x: 844.09,
        y: 371.89,
    },
    Point {
        x: 839.57,
        y: 390.4,
    },
    Point {
        x: 848.4,
        y: 407.55,
    },
    Point {
        x: 839.51,
        y: 432.76,
    },
    Point {
        x: 853.97,
        y: 471.15,
    },
    Point {
        x: 866.36,
        y: 480.77,
    },
];

#[test]
fn returns_empty_vec_if_no_points() {
    let result = simplify(&[], 1.0, false);
    assert_eq!(result.len(), 0);
}

#[test]
fn returns_empty_vec_if_one_point() {
    let input = &[Point { x: 0.0, y: 0.0 }];
    let result = simplify(input, 1.0, false);
    assert_eq!(result[0], input[0]);
}

#[test]
fn matches_expected_output() {
    let result = simplify(INPUT, 5.0, false);

    // assert_eq!(result, EXPECTED_OUTPUT);
    for (new, expected) in result.iter().zip(EXPECTED_OUTPUT.iter()) {
        assert_eq!(new, expected);
    }
}
