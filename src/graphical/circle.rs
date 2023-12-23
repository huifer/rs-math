use std::f64::consts::PI;
use log::{info};
use crate::graphical::point::Point;
use crate::graphical::rectangle::Rectangle;

#[derive(Debug, PartialEq)]
pub struct Circle {
    /// 圆的中心坐标
    pub x: f64,
    pub y: f64,
    /// 圆的半径
    pub radius: f64,
}

impl Circle {
    /// 通过两个点和半径创建圆
    pub fn from_two_points_and_radius(point1: &Point, point2: &Point, radius: f64) -> Circle {
        let h = (point1.x + point2.x) / 2.0;
        let k = (point1.y + point2.y) / 2.0;
        Circle { x: h, y: k, radius }
    }
    /// 通过两点和半径创建圆
    pub fn from_points_and_radius(point1: &Point, point2: &Point, radius: f64) -> Circle {
        // 计算圆心的中点坐标
        let center_x = (point1.x + point2.x) / 2.0;
        let center_y = (point1.y + point2.y) / 2.0;

        // 计算两点间的距离
        let distance = ((point2.x - point1.x).powi(2) + (point2.y - point1.y).powi(2)).sqrt();

        info!("圆的半径为 {}，圆心为 ({}, {})", radius, center_x, center_y);
        if radius == distance / 2.0 {
        }
        // 验证半径是否有效
        Circle {
            x: center_x,
            y: center_y,
            radius,
        }
    }
    /// 三个点算圆
    pub fn from_points(p1: &Point, p2: &Point, p3: &Point) -> Circle {
        // 计算圆心坐标 (h, k)
        let h = (p1.x + p2.x) / 2.0;
        let k = (p1.y + p2.y) / 2.0;

        // 计算半径 r
        let r = ((p1.x - h).powi(2) + (p1.y - k).powi(2)).sqrt();

        // 检查第三个点是否在圆上
        if (p3.x - h).powi(2) + (p3.y - k).powi(2) == r.powi(2) {} else {};

        Circle { x: h, y: k, radius: r }
    }
    /// 创建一个新的圆实例
    pub fn new(x: f64, y: f64, radius: f64) -> Circle {
        Circle { x, y, radius }
    }

    /// 计算圆的面积
    pub fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }

    /// 判断点是否在圆内
    pub fn is_point_inside(&self, point_x: f64, point_y: f64) -> bool {
        let distance_squared = (point_x - self.x).powi(2) + (point_y - self.y).powi(2);
        distance_squared <= self.radius.powi(2)
    }
    pub fn generate_points(&self, num_points: usize) -> Vec<Point> {
        return generate_points_on_circle(self.x, self.y, self.radius, num_points);
    }
    /// 判断点是否在圆弧上
    pub fn is_point_on_arc(&self, start_angle: f64, end_angle: f64, point: &Point) -> bool {
        let distance_squared = (point.x - self.x).powi(2) + (point.y - self.y).powi(2);
        let distance = distance_squared.sqrt();

        distance == self.radius && self.is_angle_in_range(start_angle, end_angle, point)
    }

    /// 判断夹角是否在指定范围内的辅助函数
    pub fn is_angle_in_range(&self, start_angle: f64, end_angle: f64, point: &Point) -> bool {
        let angle = (point.y - self.x).atan2(point.x - self.y);
        let positive_angle = if angle < 0.0 {
            2.0 * PI + angle
        } else {
            angle
        };
        positive_angle >= start_angle && positive_angle <= end_angle
    }

    pub fn is_point_on_circle_boundary(point: &Point, circle: &Circle) -> bool {
        let distance = distance_between_points(point.x, point.y, circle.x, circle.y);
        distance == circle.radius
    }

    /// 寻找焦点
    pub fn find_line_intersection(&self, p1: &Point, p2: &Point) -> Vec<Point> {
        let dx = p2.x - p1.x;
        let dy = p2.y - p1.y;

        let a = dx * dx + dy * dy;
        let b = 2.0 * (dx * (p1.x - self.x) + dy * (p1.y - self.y));
        let c = (p1.x - self.x) * (p1.x - self.x) + (p1.y - self.y) * (p1.y - self.y) - self.radius * self.radius;

        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            // No real solutions (no intersection)
            return Vec::new();
        }

        let t1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b - discriminant.sqrt()) / (2.0 * a);

        let mut intersections = Vec::new();

        if t1 >= 0.0 && t1 <= 1.0 {
            intersections.push(Point {
                x: p1.x + t1 * dx,
                y: p1.y + t1 * dy,
            });
        }

        if t2 >= 0.0 && t2 <= 1.0 {
            intersections.push(Point {
                x: p1.x + t2 * dx,
                y: p1.y + t2 * dy,
            });
        }

        intersections
    }
    /// 判断两个圆是否相交
    /// 它们的圆心之间的距离小于两个圆半径之和
    /// distance(center1,center2)<radius1+radius2
    pub fn circles_intersect(&self, other: &Circle) -> bool {
        let distance_between_centers = ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt();
        let sum_of_radii = self.radius + other.radius;

        distance_between_centers < sum_of_radii
    }

    /// 判断两个圆是否相切
    /// 它们的圆心之间的距离等于两个圆半径之和
    /// ∣distance(center1,center2)−(radius1+radius2)∣<ϵ
    pub fn circles_touch(&self, other: &Circle) -> bool {
        let distance_between_centers = ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt();
        let sum_of_radii = self.radius + other.radius;

        (distance_between_centers - sum_of_radii).abs() < f64::EPSILON
    }

    /// 判断一个圆是否完全包含另一个圆
    /// 它们的圆心之间的距离加上小圆半径小于等于大圆半径
    /// distance(center1,center2)+radius2≤radius1
    pub fn circle_contains(&self, other: &Circle) -> bool {
        let distance_between_centers = ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt();
        distance_between_centers + other.radius <= self.radius
    }

    /// 判断圆心是否在矩形内
    ///
    /// 公式：circle_x >= rect.x1 && circle_x <= rect.x2 && circle_y >= rect.y1 && circle_y <= rect.y2
    /// 圆心的 x 坐标在矩形的 x 范围内，且圆心的 y 坐标在矩形的 y 范围内。
    pub fn circle_inside_rectangle(&self, rect: &Rectangle) -> bool {
        self.x >= rect.x1 && self.x <= rect.x2 && self.y >= rect.y1 && self.y <= rect.y2
    }

    /// 判断圆心是否在矩形的某个边上
    ///
    /// 公式：(circle_x == rect.x1 || circle_x == rect.x2) && circle_y >= rect.y1 && circle_y <= rect.y2
    /// 或
    /// circle_x >= rect.x1 && circle_x <= rect.x2 && (circle_y == rect.y1 || circle_y == rect.y2)
    /// 圆心在矩形的 x 或 y 范围的一个边界上，但不在矩形内部。
    pub fn circle_on_rectangle_edge(&self, rect: &Rectangle) -> bool {
        (self.x == rect.x1 || self.x == rect.x2) && self.y >= rect.y1 && self.y <= rect.y2
            || self.x >= rect.x1 && self.x <= rect.x2 && (self.y == rect.y1 || self.y == rect.y2)
    }

    /// 判断圆心是否在矩形的角上
    ///
    /// 公式：(circle_x == rect.x1 && circle_y == rect.y1)
    /// 或
    /// (circle_x == rect.x1 && circle_y == rect.y2)
    /// 或
    /// (circle_x == rect.x2 && circle_y == rect.y1)
    /// 或
    /// (circle_x == rect.x2 && circle_y == rect.y2)
    /// 圆心在矩形的 x 或 y 范围的一个边界上，并且与另一个边界相交。
    pub fn circle_on_rectangle_corner(&self, rect: &Rectangle) -> bool {
        (self.x == rect.x1 && self.y == rect.y1)
            || (self.x == rect.x1 && self.y == rect.y2)
            || (self.x == rect.x2 && self.y == rect.y1)
            || (self.x == rect.x2 && self.y == rect.y2)
    }


    pub fn bounding_box(&self) -> Rectangle {
        Rectangle {
            x1: self.x - self.radius,
            y1: self.y - self.radius,
            x2: self.x + self.radius,
            y2: self.y + self.radius,
        }
    }
}

///  计算两点之间的距离
pub fn distance_between_points(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
}


pub fn generate_points_on_circle(center_x: f64, center_y: f64, radius: f64, num_points: usize) -> Vec<Point> {
    // 存储生成的点的容器
    let mut points = Vec::with_capacity(num_points);

    // 计算角度步长，确保点在圆上均匀分布
    let angle_step = 2.0 * PI / num_points as f64;

    // 生成点的坐标
    for i in 0..num_points {
        let angle = i as f64 * angle_step;
        let x = center_x + radius * angle.cos();
        let y = center_y + radius * angle.sin();
        points.push(Point { x, y });
    }

    points
}