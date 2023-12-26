#[cfg(test)]
mod tests {
    use rs_math::graphical::circle::{Circle, generate_points_on_circle, Point2D};

    #[test]
    fn test_circle_area() {
        let circle = Circle::new(0.0, 0.0, 3.0);
        assert_eq!(circle.area(), 28.274333882308138);
    }

    #[test]
    fn test_is_point_inside() {
        let circle = Circle::new(0.0, 0.0, 5.0);
        assert!(circle.is_point_inside(3.0, 4.0));
        assert!(!circle.is_point_inside(6.0, 0.0));
    }

    #[test]
    fn test_generate_points_on_circle() {
        // 测试生成 4 个点的情况
        let center_x = 0.0;
        let center_y = 0.0;
        let radius = 1.0;
        let num_points = 4;

        let points = generate_points_on_circle(center_x, center_y, radius, num_points);

        // 生成的点应该包括 (1, 0), (0, 1), (-1, 0), (0, -1)
        assert_eq!(points.len(), num_points);
    }

    #[test]
    fn test_circle_from_two_points_and_radius() {
        let point1 = Point2D { x: 0.0, y: 0.0 };
        let point2 = Point2D { x: 1.0, y: 0.0 };
        let radius = 1.0;

        let circle_from_two_points = Circle::from_two_points_and_radius(&point1, &point2, radius);
        let expected_circle = Circle { x: 0.5, y: 0.0, radius: 1.0 };

        assert_eq!(circle_from_two_points, expected_circle);
    }

    #[test]
    fn test_circle_from_three_points() {
        let point1 = Point2D { x: 0.0, y: 0.0 };
        let point2 = Point2D { x: 1.0, y: 1.0 };
        let point3 = Point2D { x: 0.0, y: 1.0 };

        let circle_from_three_points = Circle::from_points(&point1, &point2, &point3);
        let expected_circle = Circle { x: 0.5, y: 0.5, radius: 0.7071067811865476 }; // Radius is approximately sqrt(2)/2

        assert_eq!(circle_from_three_points, expected_circle);
    }

    #[test]
    fn test_valid_circle_creation() {
        let point1 = Point2D { x: 0.0, y: 0.0 };
        let point2 = Point2D { x: 4.0, y: 0.0 };
        let radius = 2.0;

        let circle = Circle::from_points_and_radius(&point1, &point2, radius);

        assert_eq!(
            circle.unwrap(),
            Circle {
                x: 2.0,
                y: 0.0,
                radius: 2.0,
            }
        );
    }

    #[test]
    fn test_circle_line_intersection() {
        let circle = Circle { x: 0.0, y: 0.0, radius: 5.0 };

        let line_point1 = Point2D { x: -8.0, y: 0.0 };
        let line_point2 = Point2D { x: 8.0, y: 0.0 };
        let intersections = circle.find_line_intersection(&line_point1, &line_point2);
        assert_eq!(intersections.len(), 2);
        assert!(intersections.contains(&Point2D { x: -5.0, y: 0.0 }));
        assert!(intersections.contains(&Point2D { x: 5.0, y: 0.0 }));


    }




}