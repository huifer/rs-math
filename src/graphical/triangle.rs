use crate::graphical::circle::Circle;
use crate::graphical::point_2d::Point2D;

#[derive(Debug, PartialEq)]
pub struct Triangle {
    pub vertex_a: Point2D,
    pub vertex_b: Point2D,
    pub vertex_c: Point2D,
}
#[derive(Debug, PartialEq)]
pub enum TriangleType {
    Acute,
    Right,
    Obtuse,
}
#[allow(dead_code)]
impl Triangle {
    // 构造函数，创建一个新的三角形实例
    pub fn new(a: Point2D, b: Point2D, c: Point2D) -> Self {
        Triangle {
            vertex_a: a,
            vertex_b: b,
            vertex_c: c,
        }
    }

    // 计算三角形的边长
    pub fn side_length(&self, start: &Point2D, end: &Point2D) -> f64 {
        ((end.x - start.x).powi(2) + (end.y - start.y).powi(2)).sqrt()
    }

    // 计算三角形的周长
    pub fn perimeter(&self) -> f64 {
        let side_ab = self.side_length(&self.vertex_a, &self.vertex_b);
        let side_bc = self.side_length(&self.vertex_b, &self.vertex_c);
        let side_ca = self.side_length(&self.vertex_c, &self.vertex_a);
        side_ab + side_bc + side_ca
    }

    // 计算三角形的面积
    pub fn area(&self) -> f64 {
        let side_ab = self.side_length(&self.vertex_a, &self.vertex_b);
        let side_bc = self.side_length(&self.vertex_b, &self.vertex_c);
        let side_ca = self.side_length(&self.vertex_c, &self.vertex_a);
        let s = self.perimeter() / 2.0; // 半周长
        (s * (s - side_ab) * (s - side_bc) * (s - side_ca)).sqrt()
    }


    // 计算三角形的半周长
    fn semi_perimeter(&self) -> f64 {
        let side_ab = self.side_length(&self.vertex_a, &self.vertex_b);
        let side_bc = self.side_length(&self.vertex_b, &self.vertex_c);
        let side_ca = self.side_length(&self.vertex_c, &self.vertex_a);
        (side_ab + side_bc + side_ca) / 2.0
    }


    // 计算内切圆
    pub fn in_circle(&self) -> Circle {
        let s = self.semi_perimeter();
        let side_ab = self.side_length(&self.vertex_a, &self.vertex_b);
        let side_bc = self.side_length(&self.vertex_b, &self.vertex_c);
        let side_ca = self.side_length(&self.vertex_c, &self.vertex_a);

        let radius = self.area() / s;

        // 计算内切圆的中心坐标
        let center_x = (side_bc * self.vertex_a.x + side_ca * self.vertex_b.x + side_ab * self.vertex_c.x) / (side_ab + side_bc + side_ca);
        let center_y = (side_bc * self.vertex_a.y + side_ca * self.vertex_b.y + side_ab * self.vertex_c.y) / (side_ab + side_bc + side_ca);

        Circle {
            x: center_x,
            y: center_y,
            radius,
        }
    }
    // 绕着指定点旋转三角形，返回新的三角形实例
    pub fn rotate_around_point(&self, center: &Point2D, angle_degrees: f64) -> Triangle {
        // 将角度转换为弧度
        let angle_radians = angle_degrees.to_radians();

        // 定义旋转矩阵
        let rotation_matrix = [
            [angle_radians.cos(), -angle_radians.sin()],
            [angle_radians.sin(), angle_radians.cos()],
        ];

        // 对每个顶点进行旋转
        let new_vertex_a = self.rotate_point(&self.vertex_a, center, rotation_matrix);
        let new_vertex_b = self.rotate_point(&self.vertex_b, center, rotation_matrix);
        let new_vertex_c = self.rotate_point(&self.vertex_c, center, rotation_matrix);

        Triangle {
            vertex_a: new_vertex_a,
            vertex_b: new_vertex_b,
            vertex_c: new_vertex_c,
        }
    }
    /// 旋转单个点
    fn rotate_point(&self, point: &Point2D, center: &Point2D, matrix: [[f64; 2]; 2]) -> Point2D {
        let translated_x = point.x - center.x;
        let translated_y = point.y - center.y;

        let rotated_x = matrix[0][0] * translated_x + matrix[0][1] * translated_y;
        let rotated_y = matrix[1][0] * translated_x + matrix[1][1] * translated_y;

        Point2D {
            x: rotated_x + center.x,
            y: rotated_y + center.y,
        }
    }
    // 判断点是否在三角形内
    pub fn point_inside_triangle(&self, p: &Point2D) -> bool {
        // 计算重心坐标
        let barycentric_coords = self.barycentric_coordinates(p);

        // 判断点是否在三角形内
        barycentric_coords.iter().all(|&coord| coord >= 0.0 && coord <= 1.0)
    }

    // 辅助方法，计算点的重心坐标
    fn barycentric_coordinates(&self, p: &Point2D) -> [f64; 3] {
        // 计算三个子三角形的面积
        let area_triangle = self.area();
        let area_sub_triangle_a = Triangle::new(p.clone(), self.vertex_b.clone(), self.vertex_c.clone()).area();
        let area_sub_triangle_b = Triangle::new(self.vertex_a.clone(), p.clone(), self.vertex_c.clone()).area();
        let area_sub_triangle_c = Triangle::new(self.vertex_a.clone(), self.vertex_b.clone(), p.clone()).area();

        // 计算重心坐标
        let p1 = area_sub_triangle_a / area_triangle;
        let p2 = area_sub_triangle_b / area_triangle;
        let p3 = area_sub_triangle_c / area_triangle;

        [p1, p2, p3]
    }

    // 计算垂心
    pub fn orthocenter(&self) -> Point2D {
        let x_h = self.vertex_a.x;
        let y_h = self.vertex_b.y;
        Point2D { x: x_h, y: y_h }
    }

    // 计算重心
    pub fn centroid(&self) -> Point2D {
        let x_g = (self.vertex_a.x + self.vertex_b.x + self.vertex_c.x) / 3.0;
        let y_g = (self.vertex_a.y + self.vertex_b.y + self.vertex_c.y) / 3.0;
        Point2D { x: x_g, y: y_g }
    }

    // 计算内心
    pub fn incenter(&self) -> Point2D {
        let a = self.vertex_b.distance_to(&self.vertex_c);
        let b = self.vertex_c.distance_to(&self.vertex_a);
        let c = self.vertex_a.distance_to(&self.vertex_b);


        let x_i = (a * self.vertex_a.x + b * self.vertex_b.x + c * self.vertex_c.x) / (a + b + c);
        let y_i = (a * self.vertex_a.y + b * self.vertex_b.y + c * self.vertex_c.y) / (a + b + c);

        Point2D { x: x_i, y: y_i }
    }

    // 计算外心
    pub fn circumcenter(&self) -> Point2D {
        let m_ab = Point2D {
            x: (self.vertex_a.x + self.vertex_b.x) / 2.0,
            y: (self.vertex_a.y + self.vertex_b.y) / 2.0,
        };

        let m_bc = Point2D {
            x: (self.vertex_b.x + self.vertex_c.x) / 2.0,
            y: (self.vertex_b.y + self.vertex_c.y) / 2.0,
        };

        let m_ca = Point2D {
            x: (self.vertex_c.x + self.vertex_a.x) / 2.0,
            y: (self.vertex_c.y + self.vertex_a.y) / 2.0,
        };

        let m_ab_slope = -(self.vertex_b.x - self.vertex_a.x) / (self.vertex_b.y - self.vertex_a.y);
        let m_bc_slope = -(self.vertex_c.x - self.vertex_b.x) / (self.vertex_c.y - self.vertex_b.y);
        let m_ca_slope = -(self.vertex_a.x - self.vertex_c.x) / (self.vertex_a.y - self.vertex_c.y);

        let x_o = (m_ab.y - m_ab_slope * m_ab.x + m_bc.y - m_bc_slope * m_bc.x + m_ca.y - m_ca_slope * m_ca.x)
            / (m_ab_slope + m_bc_slope + m_ca_slope);

        let y_o = m_ab_slope * (x_o - m_ab.x) + m_ab.y;

        Point2D { x: x_o, y: y_o }
    }

    // 计算三角形的角度
    pub fn classify_triangle(&self) -> TriangleType {
        let (angle_a, angle_b, angle_c) = self.classify_angles();

        if angle_a < 90.0 && angle_b < 90.0 && angle_c < 90.0 {
            TriangleType::Acute
        } else if angle_a == 90.0 || angle_b == 90.0 || angle_c == 90.0 {
            TriangleType::Right
        } else {
            TriangleType::Obtuse
        }
    }
    // 计算三角形的角度
    pub fn classify_angles(&self) -> (f64, f64, f64) {
        let angle_a = self.angle_between(&self.vertex_b, &self.vertex_c, &self.vertex_a);
        let angle_b = self.angle_between(&self.vertex_a, &self.vertex_c, &self.vertex_b);
        let angle_c = self.angle_between(&self.vertex_a, &self.vertex_b, &self.vertex_c);

        (angle_a, angle_b, angle_c)
    }

    // 计算两条边之间的角度
    fn angle_between(&self, p1: &Point2D, p2: &Point2D, p3: &Point2D) -> f64 {
        let a = p1.distance_to(p2);
        let b = p2.distance_to(p3);
        let c = p3.distance_to(p1);

        let cos_angle = (a.powi(2) + b.powi(2) - c.powi(2)) / (2.0 * a * b);
        let angle_rad = cos_angle.acos();

        // 将弧度转换为角度
        let angle_deg = angle_rad.to_degrees();

        angle_deg
    }
}