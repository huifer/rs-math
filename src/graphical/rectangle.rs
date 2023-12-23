use crate::graphical::circle::Circle;

#[derive(Debug, PartialEq)]
pub struct Rectangle {
    pub x1: f64,
    pub y1: f64,
    pub x2: f64,
    pub y2: f64,
}


impl Rectangle {
    ///
    pub fn new_from_corner(x: f64, y: f64, width: f64, height: f64) -> Self {
        match (x, y) {
            (x1, y1) if x1 + width <= x && y1 + height <= y => {
                // 左上角
                Rectangle {
                    x1: x - width,
                    y1: y - height,
                    x2: x,
                    y2: y,
                }
            }
            (x1, y1) if x1 <= x && y1 <= y => {
                // 左下角
                Rectangle {
                    x1: x1,
                    y1: y1 - height,
                    x2: x1 + width,
                    y2: y,
                }
            }
            (x1, y1) if x1 + width >= x && y1 <= y => {
                // 右上角
                Rectangle {
                    x1: x,
                    y1: y1 - height,
                    x2: x + width,
                    y2: y,
                }
            }
            (x1, y1) if x1 <= x && y1 + height >= y => {
                // 右下角
                Rectangle {
                    x1: x1,
                    y1: y,
                    x2: x1 + width,
                    y2: y + height,
                }
            }
            _ => panic!("Invalid corner point"),
        }
    }

    /// 缩放矩形
    pub fn scale(&mut self, factor: f64) {
        self.x1 *= factor;
        self.y1 *= factor;
        self.x2 *= factor;
        self.y2 *= factor;
    }


    /// 求四个顶点
    fn rectangle_corners(r: &Rectangle) -> (Point, Point, Point, Point) {
        (
            // 左上角
            Point { x: r.x1, y: r.y1 },
            // 右上角
            Point { x: r.x2, y: r.y1 },
            // 左下角
            Point { x: r.x1, y: r.y2 },
            // 右下角
            Point { x: r.x2, y: r.y2 },
        )
    }


    /// 每个点都旋转一个角度
    fn rotate_rectangle(rect: &Rectangle, angle: f64) -> Rectangle {
        let cos_theta = angle.cos();
        let sin_theta = angle.sin();

        let new_x1 = rect.x1 * cos_theta - rect.y1 * sin_theta;
        let new_y1 = rect.x1 * sin_theta + rect.y1 * cos_theta;
        let new_x2 = rect.x2 * cos_theta - rect.y2 * sin_theta;
        let new_y2 = rect.x2 * sin_theta + rect.y2 * cos_theta;

        Rectangle {
            x1: new_x1,
            y1: new_y1,
            x2: new_x2,
            y2: new_y2,
        }
    }
    /// 计算矩形的面积
    fn area(&self) -> f64 {
        (self.x2 - self.x1) * (self.y2 - self.y1)
    }

    /// 根据点旋转
    fn rotate_rectangle_with_point(rect: &Rectangle, center: Point, angle: f64) -> Rectangle {
        // 计算旋转矩阵的元素
        let cos_theta = angle.cos();
        let sin_theta = angle.sin();

        // 计算旋转后的矩形的四个顶点坐标
        let new_x1 = (rect.x1 - center.x) * cos_theta - (rect.y1 - center.y) * sin_theta + center.x;
        let new_y1 = (rect.x1 - center.x) * sin_theta + (rect.y1 - center.y) * cos_theta + center.y;
        let new_x2 = (rect.x2 - center.x) * cos_theta - (rect.y2 - center.y) * sin_theta + center.x;
        let new_y2 = (rect.x2 - center.x) * sin_theta + (rect.y2 - center.y) * cos_theta + center.y;

        // 创建并返回旋转后的矩形
        Rectangle { x1: new_x1, y1: new_y1, x2: new_x2, y2: new_y2 }
    }

    pub fn inscribed_circle(&self) -> Circle {
        let (x, y) = &self.center();
        Circle {
            x: *x,
            y: *y,
            radius: self.inscribed_circle_radius(),
        }
    }
    // 计算矩形的中心点坐标
    fn center(&self) -> (f64, f64) {
        let center_x = (self.x1 + self.x2) / 2.0;
        let center_y = (self.y1 + self.y2) / 2.0;
        (center_x, center_y)
    }

    // 计算内切圆的半径
    fn inscribed_circle_radius(&self) -> f64 {
        let width = self.x2 - self.x1;
        let height = self.y2 - self.y1;

        std::cmp::min(width, height) / 2.0
    }
}

