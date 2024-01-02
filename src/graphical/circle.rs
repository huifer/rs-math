use std::f64::consts::PI;
use crate::graphical::point_2d::Point2D;
use crate::graphical::rectangle::Rectangle;

/// 表示一个圆的结构体。
#[derive(Debug, PartialEq)]
pub struct Circle {
    /// 圆的中心坐标
    pub x: f64,
    pub y: f64,
    /// 圆的半径
    pub radius: f64,
}

#[allow(dead_code)]
impl Circle {
    /// 通过两点和半径创建圆。
    ///
    /// # 参数
    ///
    /// * `point1` - 圆上的第一个点。
    /// * `point2` - 圆上的第二个点。
    /// * `radius` - 圆的半径。
    ///
    /// # 返回值
    ///
    /// 如果给定半径有效，返回一个包含圆心和半径的 `Circle` 结构体实例；否则返回 `None`。
    ///
    /// # 示例
    ///
    /// ```
    /// use rs_math::graphical::point_2d::Point2D;
    /// use rs_math::graphical::circle::Circle;
    ///
    /// let point1 = Point2D { x: 0.0, y: 0.0 };
    /// let point2 = Point2D { x: 1.0, y: 0.0 };
    /// let radius = 0.5;
    /// let circle = Circle::from_points_and_radius(&point1, &point2, radius);
    /// ```
    pub fn from_points_and_radius(point1: &Point2D, point2: &Point2D, radius: f64) -> Option<Circle> {
        // 计算圆心的中点坐标
        let center_x = (point1.x + point2.x) / 2.0;
        let center_y = (point1.y + point2.y) / 2.0;

        // 计算两点间的距离
        let distance = ((point2.x - point1.x).powi(2) + (point2.y - point1.y).powi(2)).sqrt();

        // 验证半径是否有效
        if (radius - distance / 2.0).abs() < std::f64::EPSILON {
            Some(Circle {
                x: center_x,
                y: center_y,
                radius,
            })
        } else {
            None
        }
    }
    /// 通过三个点创建圆。
    ///
    /// # 参数
    ///
    /// * `p1` - 圆上的第一个点。
    /// * `p2` - 圆上的第二个点。
    /// * `p3` - 圆上的第三个点。
    ///
    /// # 返回值
    ///
    /// 如果给定三个点共线，返回 `None`；否则返回一个包含圆心和半径的 `Circle` 结构体实例。
    ///
    /// # 示例
    ///
    /// ```
    /// use rs_math::graphical::point_2d::Point2D;
    /// use rs_math::graphical::circle::Circle;
    ///
    /// let p1 = Point2D { x: 0.0, y: 0.0 };
    /// let p2 = Point2D { x: 1.0, y: 0.0 };
    /// let p3 = Point2D { x: 0.0, y: 1.0 };
    /// let circle = Circle::from_points(&p1, &p2, &p3);
    /// ```
    pub fn from_points(p1: &Point2D, p2: &Point2D, p3: &Point2D) -> Option<Circle> {
        // 计算圆心坐标 (h, k)
        let h = (p1.x + p2.x) / 2.0;
        let k = (p1.y + p2.y) / 2.0;

        // 计算半径 r
        let r = ((p1.x - h).powi(2) + (p1.y - k).powi(2)).sqrt();

        // 检查第三个点是否在圆上
        let distance_to_center_squared = (p3.x - h).powi(2) + (p3.y - k).powi(2);
        let epsilon = 1e-6; // 设置一个小的误差范围

        // 如果给定三个点共线，返回 None；否则返回包含圆心和半径的 Circle 结构体实例。
        if (distance_to_center_squared - r.powi(2)).abs() < epsilon {
            Some(Circle { x: h, y: k, radius: r })
        } else {
            None
        }
    }
    /// 创建一个新的圆实例。
    ///
    /// # 参数
    ///
    /// * `x` - 圆的中心横坐标。
    /// * `y` - 圆的中心纵坐标。
    /// * `radius` - 圆的半径。
    ///
    /// # 返回值
    ///
    /// 返回一个包含给定圆心和半径的 `Circle` 结构体实例。
    ///
    /// # 示例
    ///
    /// ```
    /// use rs_math::graphical::circle::Circle;
    ///
    /// let circle = Circle::new(0.0, 0.0, 1.0);
    /// ```
    pub fn new(x: f64, y: f64, radius: f64) -> Circle {
        Circle { x, y, radius }
    }

    /// 计算圆的面积。
    ///
    /// # 返回值
    ///
    /// 返回圆的面积，使用标准的 π（pi） 值。
    ///
    /// # 示例
    ///
    /// ```
    /// use rs_math::graphical::circle::Circle;
    ///
    /// let circle = Circle::new(0.0, 0.0, 1.0);
    /// let area = circle.area();
    /// ```
    pub fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }

    /// 判断给定点是否在圆内。
    ///
    /// # 参数
    ///
    /// * `point_x` - 待判断点的横坐标。
    /// * `point_y` - 待判断点的纵坐标。
    ///
    /// # 返回值
    ///
    /// 如果给定点在圆内或在圆上，返回 `true`；否则返回 `false`。
    ///
    /// # 示例
    ///
    /// ```
    /// use rs_math::graphical::circle::Circle;
    ///
    /// let circle = Circle::new(0.0, 0.0, 1.0);
    /// let is_inside = circle.is_point_inside(0.5, 0.5);
    /// ```
    pub fn is_point_inside(&self, point_x: f64, point_y: f64) -> bool {
        let distance_squared = (point_x - self.x).powi(2) + (point_y - self.y).powi(2);
        distance_squared <= self.radius.powi(2)
    }
    /// 生成圆上的点。
    ///
    /// # 参数
    ///
    /// * `num_points` - 要生成的点的数量。
    ///
    /// # 返回值
    ///
    /// 返回一个包含圆上生成点的 `Vec<Point2D>`。
    ///
    /// # 示例
    ///
    /// ```
    /// use rs_math::graphical::circle::Circle;
    /// use rs_math::graphical::point_2d::Point2D;
    ///
    /// let circle = Circle::new(0.0, 0.0, 1.0);
    /// let points = circle.generate_points(10);
    /// ```
    pub fn generate_points(&self, num_points: usize) -> Vec<Point2D> {
        generate_points_on_circle(self.x, self.y, self.radius, num_points)
    }
    /// 判断点是否在圆弧上。
    ///
    /// # 参数
    ///
    /// * `start_angle` - 圆弧的起始角度。
    /// * `end_angle` - 圆弧的结束角度。
    /// * `point` - 待判断的点。
    ///
    /// # 返回值
    ///
    /// 如果给定点在圆弧上，返回 `true`；否则返回 `false`。
    ///
    /// # 示例
    ///
    /// ```
    /// use rs_math::graphical::circle::Circle;
    /// use rs_math::graphical::point_2d::Point2D;
    ///
    /// let circle = Circle::new(0.0, 0.0, 1.0);
    /// let start_angle = 0.0;
    /// let end_angle = std::f64::consts::PI;
    /// let point = Point2D { x: 1.0, y: 0.0 };
    /// let is_on_arc = circle.is_point_on_arc(start_angle, end_angle, &point);
    /// ```
    pub fn is_point_on_arc(&self, start_angle: f64, end_angle: f64, point: &Point2D) -> bool {
        let distance_squared = (point.x - self.x).powi(2) + (point.y - self.y).powi(2);
        let distance = distance_squared.sqrt();

        distance == self.radius && self.is_angle_in_range(start_angle, end_angle, point)
    }

    /// 判断夹角是否在指定范围内的辅助函数。
    ///
    /// # 参数
    ///
    /// * `start_angle` - 范围的起始角度。
    /// * `end_angle` - 范围的结束角度。
    /// * `point` - 待判断的点。
    ///
    /// # 返回值
    ///
    /// 如果给定点的夹角在指定范围内，返回 `true`；否则返回 `false`。
    ///
    /// # 示例
    ///
    /// ```
    /// use rs_math::graphical::circle::Circle;
    /// use rs_math::graphical::point_2d::Point2D;
    ///
    /// let circle = Circle::new(0.0, 0.0, 1.0);
    /// let start_angle = 0.0;
    /// let end_angle = std::f64::consts::PI;
    /// let point = Point2D { x: 1.0, y: 0.0 };
    /// let is_in_range = circle.is_angle_in_range(start_angle, end_angle, &point);
    /// ```
    pub fn is_angle_in_range(&self, start_angle: f64, end_angle: f64, point: &Point2D) -> bool {
        let angle = (point.y - self.x).atan2(point.x - self.y);
        let positive_angle = if angle < 0.0 {
            2.0 * std::f64::consts::PI + angle
        } else {
            angle
        };
        positive_angle >= start_angle && positive_angle <= end_angle
    }

    /// 判断点是否在圆边界上。
    ///
    /// # 参数
    ///
    /// * `point` - 待判断的点。
    ///
    /// # 返回值
    ///
    /// 如果给定点在圆边界上，返回 `true`；否则返回 `false`。
    ///
    /// # 示例
    ///
    /// ```
    /// use rs_math::graphical::circle::Circle;
    /// use rs_math::graphical::point_2d::Point2D;
    ///
    /// let circle = Circle::new(0.0, 0.0, 1.0);
    /// let point = Point2D { x: 1.0, y: 0.0 };
    /// let is_on_boundary = circle.is_point_on_circle_boundary(&point);
    /// ```
    pub fn is_point_on_circle_boundary(&self, point: &Point2D) -> bool {
        let distance = distance_between_points(point.x, point.y, self.x, self.y);
        distance == self.radius
    }

    /// 寻找与直线的交点。
    ///
    /// # 参数
    ///
    /// * `p1` - 直线上的第一个点。
    /// * `p2` - 直线上的第二个点。
    ///
    /// # 返回值
    ///
    /// 返回一个包含交点的 `Vec<Point2D>`。
    ///
    /// # 示例
    ///
    /// ```
    /// use rs_math::graphical::circle::Circle;
    /// use rs_math::graphical::point_2d::Point2D;
    ///
    /// let circle = Circle::new(0.0, 0.0, 1.0);
    /// let point1 = Point2D { x: -2.0, y: 0.0 };
    /// let point2 = Point2D { x: 2.0, y: 0.0 };
    /// let intersections = circle.find_line_intersection(&point1, &point2);
    /// ```
    pub fn find_line_intersection(&self, p1: &Point2D, p2: &Point2D) -> Vec<Point2D> {
        let dx = p2.x - p1.x;
        let dy = p2.y - p1.y;

        let a = dx * dx + dy * dy;
        let b = 2.0 * (dx * (p1.x - self.x) + dy * (p1.y - self.y));
        let c = (p1.x - self.x) * (p1.x - self.x) + (p1.y - self.y) * (p1.y - self.y) - self.radius * self.radius;

        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            // 无实数解（无交点）
            return Vec::new();
        }

        let t1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b - discriminant.sqrt()) / (2.0 * a);

        let mut intersections = Vec::new();

        if t1 >= 0.0 && t1 <= 1.0 {
            intersections.push(Point2D {
                x: p1.x + t1 * dx,
                y: p1.y + t1 * dy,
            });
        }

        if t2 >= 0.0 && t2 <= 1.0 {
            intersections.push(Point2D {
                x: p1.x + t2 * dx,
                y: p1.y + t2 * dy,
            });
        }

        intersections
    }
    /// 判断两个圆是否相交。
    ///
    /// # 参数
    ///
    /// * `other` - 另一个圆的实例。
    ///
    /// # 返回值
    ///
    /// 如果两个圆相交，返回 `true`；否则返回 `false`。
    ///
    /// # 示例
    ///
    /// ```
    /// use rs_math::graphical::circle::Circle;
    ///
    /// let circle1 = Circle::new(0.0, 0.0, 1.0);
    /// let circle2 = Circle::new(2.0, 0.0, 1.0);
    /// let do_intersect = circle1.circles_intersect(&circle2);
    /// ```
    pub fn circles_intersect(&self, other: &Circle) -> bool {
        let distance_between_centers = ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt();
        let sum_of_radii = self.radius + other.radius;

        distance_between_centers < sum_of_radii
    }

    /// 判断两个圆是否相切。
    ///
    /// # 参数
    ///
    /// * `other` - 另一个圆的实例。
    ///
    /// # 返回值
    ///
    /// 如果两个圆相切，返回 `true`；否则返回 `false`。
    ///
    /// # 示例
    ///
    /// ```
    /// use rs_math::graphical::circle::Circle;
    ///
    /// let circle1 = Circle::new(0.0, 0.0, 1.0);
    /// let circle2 = Circle::new(2.0, 0.0, 1.0);
    /// let do_touch = circle1.circles_touch(&circle2);
    /// ```
    pub fn circles_touch(&self, other: &Circle) -> bool {
        let distance_between_centers = ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt();
        let sum_of_radii = self.radius + other.radius;

        (distance_between_centers - sum_of_radii).abs() < f64::EPSILON
    }

    /// 判断一个圆是否完全包含另一个圆。
    ///
    /// # 参数
    ///
    /// * `other` - 另一个圆的实例。
    ///
    /// # 返回值
    ///
    /// 如果一个圆完全包含另一个圆，返回 `true`；否则返回 `false`。
    ///
    /// # 示例
    ///
    /// ```
    /// use rs_math::graphical::circle::Circle;
    ///
    /// let large_circle = Circle::new(0.0, 0.0, 2.0);
    /// let small_circle = Circle::new(0.0, 0.0, 1.0);
    /// let does_contain = large_circle.circle_contains(&small_circle);
    /// ```
    pub fn circle_contains(&self, other: &Circle) -> bool {
        let distance_between_centers = ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt();
        distance_between_centers + other.radius <= self.radius
    }

    /// 判断圆心是否在矩形内。
    ///
    /// 公式：`circle_x >= rect.x1 && circle_x <= rect.x2 && circle_y >= rect.y1 && circle_y <= rect.y2`
    ///
    /// 圆心的 x 坐标在矩形的 x 范围内，且圆心的 y 坐标在矩形的 y 范围内。
    ///
    /// # 参数
    ///
    /// * `rect` - 包含矩形的实例。
    ///
    /// # 返回值
    ///
    /// 如果圆心在矩形内，返回 `true`；否则返回 `false`。
    ///
    /// # 示例
    ///
    /// ```
    /// use rs_math::graphical::circle::Circle;
    /// use rs_math::graphical::rectangle::Rectangle;
    ///
    /// let circle = Circle::new(1.0, 1.0, 2.0);
    /// let rectangle = Rectangle::new(0.0, 0.0, 3.0, 3.0);
    /// let is_inside = circle.circle_inside_rectangle(&rectangle);
    /// ```
    pub fn circle_inside_rectangle(&self, rect: &Rectangle) -> bool {
        self.x >= rect.x1 && self.x <= rect.x2 && self.y >= rect.y1 && self.y <= rect.y2
    }

    /// 判断圆心是否在矩形的某个边上。
    ///
    /// 公式：
    /// - `(circle_x == rect.x1 || circle_x == rect.x2) && circle_y >= rect.y1 && circle_y <= rect.y2`
    /// 或
    /// - `circle_x >= rect.x1 && circle_x <= rect.x2 && (circle_y == rect.y1 || circle_y == rect.y2)`
    ///
    /// 圆心在矩形的 x 或 y 范围的一个边界上，但不在矩形内部。
    ///
    /// # 参数
    ///
    /// * `rect` - 包含矩形的实例。
    ///
    /// # 返回值
    ///
    /// 如果圆心在矩形的边上，返回 `true`；否则返回 `false`。
    ///
    /// # 示例
    ///
    /// ```
    /// use rs_math::graphical::circle::Circle;
    /// use rs_math::graphical::rectangle::Rectangle;
    ///
    /// let circle = Circle::new(2.0, 2.0, 1.0);
    /// let rectangle = Rectangle::new(1.0, 1.0, 3.0, 3.0);
    /// let on_edge = circle.circle_on_rectangle_edge(&rectangle);
    /// ```
    pub fn circle_on_rectangle_edge(&self, rect: &Rectangle) -> bool {
        (self.x == rect.x1 || self.x == rect.x2) && self.y >= rect.y1 && self.y <= rect.y2 || self.x >= rect.x1 && self.x <= rect.x2 && (self.y == rect.y1 || self.y == rect.y2)
    }

    /// 判断圆心是否在矩形的角上。
    ///
    /// 公式：
    /// - `(circle_x == rect.x1 && circle_y == rect.y1)`
    /// 或
    /// - `(circle_x == rect.x1 && circle_y == rect.y2)`
    /// 或
    /// - `(circle_x == rect.x2 && circle_y == rect.y1)`
    /// 或
    /// - `(circle_x == rect.x2 && circle_y == rect.y2)`
    ///
    /// 圆心在矩形的 x 或 y 范围的一个边界上，并且与另一个边界相交。
    ///
    /// # 参数
    ///
    /// * `rect` - 包含矩形的实例。
    ///
    /// # 返回值
    ///
    /// 如果圆心在矩形的角上，返回 `true`；否则返回 `false`。
    ///
    /// # 示例
    ///
    /// ```
    /// use rs_math::graphical::circle::Circle;
    /// use rs_math::graphical::rectangle::Rectangle;
    ///
    /// let circle = Circle::new(1.0, 1.0, 0.5);
    /// let rectangle = Rectangle::new(0.0, 0.0, 2.0, 2.0);
    /// let on_corner = circle.circle_on_rectangle_corner(&rectangle);
    /// ```
    pub fn circle_on_rectangle_corner(&self, rect: &Rectangle) -> bool {
        (self.x == rect.x1 && self.y == rect.y1) || (self.x == rect.x1 && self.y == rect.y2) || (self.x == rect.x2 && self.y == rect.y1) || (self.x == rect.x2 && self.y == rect.y2)
    }

    /// 获取圆的外接矩形。
    ///
    /// 外接矩形的左上角坐标为 `(circle_x - radius, circle_y - radius)`，
    /// 右下角坐标为 `(circle_x + radius, circle_y + radius)`。
    ///
    /// # 返回值
    ///
    /// 返回一个包含外接矩形坐标的 `Rectangle` 结构体实例。
    ///
    /// # 示例
    ///
    /// ```
    /// use rs_math::graphical::circle::Circle;
    /// use rs_math::graphical::rectangle::Rectangle;
    ///
    /// let circle = Circle::new(3.0, 4.0, 2.0);
    /// let bounding_box = circle.bounding_box();
    /// ```

    pub fn bounding_box(&self) -> Rectangle {
        Rectangle {
            x1: self.x - self.radius,
            y1: self.y - self.radius,
            x2: self.x + self.radius,
            y2: self.y + self.radius,
        }
    }
}
/// 计算两点之间的欧几里德距离。
///
/// 公式：sqrt((x2 - x1)^2 + (y2 - y1)^2)
///
/// # 参数
///
/// * `x1` - 第一个点的 x 坐标。
/// * `y1` - 第一个点的 y 坐标。
/// * `x2` - 第二个点的 x 坐标。
/// * `y2` - 第二个点的 y 坐标。
///
/// # 返回值
///
/// 返回两点之间的欧几里德距离。
///
/// # 示例
///
/// ```
/// use rs_math::graphical::circle::distance_between_points;
///
/// let distance = distance_between_points(1.0, 2.0, 4.0, 6.0);
/// ```
pub fn distance_between_points(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
}

/// 生成圆上的均匀分布的点集合。
///
/// # 参数
///
/// * `center_x` - 圆心的 x 坐标。
/// * `center_y` - 圆心的 y 坐标。
/// * `radius` - 圆的半径。
/// * `num_points` - 生成的点的数量。
///
/// # 返回值
///
/// 返回一个包含生成的点坐标的 `Vec<Point2D>` 实例。
///
/// # 示例
///
/// ```
/// use rs_math::graphical::circle::generate_points_on_circle;
///
/// let points = generate_points_on_circle(0.0, 0.0, 1.0, 8);
/// ```

pub fn generate_points_on_circle(center_x: f64, center_y: f64, radius: f64, num_points: usize) -> Vec<Point2D> {
    // 存储生成的点的容器
    let mut points = Vec::with_capacity(num_points);

    // 计算角度步长，确保点在圆上均匀分布
    let angle_step = 2.0 * PI / num_points as f64;

    // 生成点的坐标
    for i in 0..num_points {
        let angle = i as f64 * angle_step;
        let x = center_x + radius * angle.cos();
        let y = center_y + radius * angle.sin();
        points.push(Point2D { x, y });
    }

    points
}