use crate::graphical::circle::Circle;
use crate::graphical::point_2d::Point2D;

/// 表示二维平面上的矩形的结构体。
///
/// 矩形由左上角顶点 (`x1`, `y1`) 和右下角顶点 (`x2`, `y2`) 定义。
#[derive(Debug, PartialEq)]
pub struct Rectangle {
    /// 左上角顶点的 x 坐标。
    pub x1: f64,
    /// 左上角顶点的 y 坐标。
    pub y1: f64,
    /// 右下角顶点的 x 坐标。
    pub x2: f64,
    /// 右下角顶点的 y 坐标。
    pub y2: f64,
}


#[allow(dead_code)]
impl Rectangle {
    /// 通过给定的左上角顶点坐标、宽度和高度构建矩形。
    ///
    /// # 参数
    ///
    /// - `x`: 左上角顶点的 x 坐标。
    /// - `y`: 左上角顶点的 y 坐标。
    /// - `width`: 矩形的宽度。
    /// - `height`: 矩形的高度。
    ///
    /// # 返回
    ///
    /// 返回通过左上角顶点坐标、宽度和高度定义的矩形。
    ///
    /// # 注意
    ///
    /// 如果提供的参数不合法，将会触发 panic。
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

    /// 缩放矩形。
    ///
    /// # 参数
    ///
    /// - `sx`: x 方向的缩放比例。
    /// - `sy`: y 方向的缩放比例。
    ///
    /// # 注意
    ///
    /// 该方法会修改矩形的右下角顶点坐标，使矩形按照指定的比例进行缩放。
    pub fn scale(&mut self, sx: f64, sy: f64) {
        // 计算新的右下角顶点坐标
        self.x2 = self.x1 + sx * (self.x2 - self.x1);
        self.y2 = self.y1 + sy * (self.y2 - self.y1);
    }

    /// 求矩形的四个顶点。
    ///
    /// # 参数
    ///
    /// - `r`: 给定的矩形。
    ///
    /// # 返回
    ///
    /// 返回矩形的四个顶点，按照左上角、右上角、左下角、右下角的顺序排列。
    fn rectangle_corners(r: &Rectangle) -> (Point2D, Point2D, Point2D, Point2D) {
        (
            // 左上角
            Point2D { x: r.x1, y: r.y1 },
            // 右上角
            Point2D { x: r.x2, y: r.y1 },
            // 左下角
            Point2D { x: r.x1, y: r.y2 },
            // 右下角
            Point2D { x: r.x2, y: r.y2 },
        )
    }


    /// 旋转一个矩形（绕原点）。
    ///
    /// # 参数
    ///
    /// - `angle`: 旋转角度（弧度）。
    ///
    /// # 返回
    ///
    /// 返回一个新的矩形，表示原矩形绕原点逆时针旋转一定角度后的状态。
    pub fn rotate(&self, angle: f64) -> Rectangle {
        // 计算旋转后的左上角顶点坐标
        let new_x1 = self.x1 * angle.cos() - self.y1 * angle.sin();
        let new_y1 = self.x1 * angle.sin() + self.y1 * angle.cos();

        // 计算旋转后的右下角顶点坐标
        let new_x2 = self.x2 * angle.cos() - self.y2 * angle.sin();
        let new_y2 = self.x2 * angle.sin() + self.y2 * angle.cos();

        Rectangle {
            x1: new_x1,
            y1: new_y1,
            x2: new_x2,
            y2: new_y2,
        }
    }
    /// 计算矩形的面积。
    ///
    /// # 返回
    ///
    /// 返回矩形的面积。
    pub fn area(&self) -> f64 {
        (self.x2 - self.x1) * (self.y2 - self.y1)
    }


    /// 计算矩形的周长。
    ///
    /// # 返回
    ///
    /// 返回矩形的周长。
    pub fn perimeter(&self) -> f64 {
        // 计算矩形的宽度和高度
        let width = self.x2 - self.x1;
        let height = self.y2 - self.y1;

        // 计算矩形的周长
        let perimeter = 2.0 * (width + height);

        perimeter.abs() // 确保周长为正数
    }

    /// 判断点是否在矩形内。
    ///
    /// # 参数
    ///
    /// - `x`: 点的 x 坐标。
    /// - `y`: 点的 y 坐标。
    ///
    /// # 返回
    ///
    /// 如果点在矩形内，返回 `true`，否则返回 `false`。
    pub fn point_inside(&self, x: f64, y: f64) -> bool {
        x >= self.x1 && x <= self.x2 && y >= self.y1 && y <= self.y2
    }
    /// 根据点旋转矩形。
    ///
    /// # 参数
    ///
    /// - `angle`: 旋转角度（弧度）。
    /// - `x_rot`: 旋转点的 x 坐标。
    /// - `y_rot`: 旋转点的 y 坐标。
    ///
    /// # 返回
    ///
    /// 旋转后的矩形。
    pub fn rotate_around_point(&self, angle: f64, x_rot: f64, y_rot: f64) -> Rectangle {
        // 使用旋转矩阵对矩形进行旋转
        let x1_prime = (self.x1 - x_rot) * angle.cos() - (self.y1 - y_rot) * angle.sin();
        let y1_prime = (self.x1 - x_rot) * angle.sin() + (self.y1 - y_rot) * angle.cos();
        let x2_prime = (self.x2 - x_rot) * angle.cos() - (self.y2 - y_rot) * angle.sin();
        let y2_prime = (self.x2 - x_rot) * angle.sin() + (self.y2 - y_rot) * angle.cos();

        // 将旋转后的中心点坐标平移到旋转点
        let x1 = x1_prime + x_rot;
        let y1 = y1_prime + y_rot;
        let x2 = x2_prime + x_rot;
        let y2 = y2_prime + y_rot;

        Rectangle { x1, y1, x2, y2 }
    }

    /// 判断两个矩形是否相交。
    ///
    /// # 参数
    ///
    /// - `other`: 另一个矩形。
    ///
    /// # 返回
    ///
    /// 如果两个矩形相交，则返回 `true`，否则返回 `false`。
    pub fn intersect(&self, other: &Rectangle) -> bool {
        self.x1 <= other.x2 && self.x2 >= other.x1 && self.y1 <= other.y2 && self.y2 >= other.y1
    }

    /// 判断当前矩形是否包含另一个矩形。
    ///
    /// # 参数
    ///
    /// - `other`: 要检查的矩形。
    ///
    /// # 返回
    ///
    /// 如果当前矩形包含另一个矩形，则返回 `true`，否则返回 `false`。
    pub fn contains(&self, other: &Rectangle) -> bool {
        self.x1 <= other.x1 && self.x2 >= other.x2 && self.y1 <= other.y1 && self.y2 >= other.y2
    }
    /// 计算矩形的内切圆。
    ///
    /// # 返回
    ///
    /// 返回一个包含内切圆信息的 `Circle` 结构体。
    pub fn inscribed_circle(&self) -> Circle {
        let (x, y) = &self.center();
        Circle {
            x: *x,
            y: *y,
            radius: self.inscribed_circle_radius(),
        }
    }
    /// 计算矩形的中心点坐标。
    ///
    /// # 返回
    ///
    /// 返回一个包含矩形中心点坐标 `(x, y)` 的元组。
    fn center(&self) -> (f64, f64) {
        let center_x = (self.x1 + self.x2) / 2.0;
        let center_y = (self.y1 + self.y2) / 2.0;
        (center_x, center_y)
    }

    /// 计算内切圆的半径。
    ///
    /// # 返回
    ///
    /// 返回内切圆的半径，即矩形宽度和高度中较小值的一半。
    fn inscribed_circle_radius(&self) -> f64 {
        let width = self.x2 - self.x1;
        let height = self.y2 - self.y1;

        f64::min(width, height) / 2.0
    }
}

