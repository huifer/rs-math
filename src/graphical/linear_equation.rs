use std::f64::consts::PI;
use std::f64::EPSILON;
use crate::graphical::circle::Circle;
use crate::graphical::point_2d::Point2D;

// 直线方程
#[derive(Debug)]
pub struct LinearEquation {
    pub a: f64,
    pub b: f64,
    pub c: f64,
}

#[derive(Debug, PartialEq, Clone)]
pub enum PointLineRelationship {
    OnLine,
    AboveLine,
    BelowLine,
}
#[allow(dead_code)]
impl LinearEquation {
    pub fn to_string(&self) -> String {
        format!("{}x + {}y + {} = 0", self.a, self.b, self.c)
    }
    // 通过两点计算切线方程的一般式表示
    pub fn from_points(x1: f64, y1: f64, x2: f64, y2: f64) -> LinearEquation {
        let a = y2 - y1;
        let b = x1 - x2;
        let c = x2 * y1 - x1 * y2;

        LinearEquation {
            a,
            b,
            c,
        }
    }
    // 从点斜式参数创建一般式方程
    // x1, y1 是直线上的一点
    // slope 是直线的斜率
    pub fn from_point_slope(x1: f64, y1: f64, slope: f64) -> Self {
        // 一般式方程的 A 系数为 -slope
        let a = -slope;
        // 一般式方程的 B 系数为 1
        let b = 1.0;
        // 一般式方程的 C 系数为 y1 - slope * x1
        let c = y1 - slope * x1;

        // 创建并返回一般式方程的实例
        LinearEquation { a, b, c }
    }
    // 检查两条直线是否平行
    pub fn is_parallel_to(&self, other: &LinearEquation) -> bool {
        // 两条直线平行的条件是它们的斜率相等
        if let (Some(slope1), Some(slope2)) = (self.slope(), other.slope()) {
            slope1 == slope2
        } else {
            false // 如果其中一条直线的斜率无法计算，则无法判断是否平行
        }
    }

    // 将截距式方程转换为一般式方程
    pub fn from_slope_intercept(m: f64, b: f64) -> Self {
        // 一般式方程的 A 系数为 -m
        let a = -m;
        // 一般式方程的 B 系数为 1
        // 一般式方程的 C 系数为 -b
        let c = -b;

        // 创建并返回一般式方程的实例
        LinearEquation { a, b: 1.0, c }
    }

    /// 计算圆弧切线方程的一般式表示
    pub fn from_arc(_radius: f64, x1: f64, y1: f64, x2: f64, y2: f64) -> LinearEquation {
        // 计算圆心坐标
        let center_x = (x1 + x2) / 2.0;
        let center_y = (y1 + y2) / 2.0;

        // 计算切线斜率
        let m = (y2 - y1) / (x2 - x1);

        // 计算切线方程的斜率和截距
        let a = -m;
        let b = 1.0;
        let c = -a * center_x - b * center_y;

        LinearEquation { a: a, b: b, c: c }
    }
    // 将直线沿 x 轴平移 h 个单位，返回新的直线方程
    pub fn translate_along_x(&self, h: f64) -> LinearEquation {
        LinearEquation {
            a: self.a,
            b: self.b,
            c: self.c + h,
        }
    }

    // 将直线沿 y 轴平移 k 个单位，返回新的直线方程
    pub fn translate_along_y(&self, k: f64) -> LinearEquation {
        LinearEquation {
            a: self.a,
            b: self.b,
            c: self.c + k,
        }
    }

    // 将直线绕原点逆时针旋转 theta 弧度，返回新的直线方程
    pub fn rotate_around_origin(&self, theta: f64) -> LinearEquation {
        let cos_theta = theta.cos();
        let sin_theta = theta.sin();

        // 定义旋转矩阵
        let rotation_matrix = [
            [cos_theta, -sin_theta],
            [sin_theta, cos_theta],
        ];

        // 计算新的系数
        let new_a = self.a * rotation_matrix[0][0] + self.b * rotation_matrix[0][1];
        let new_b = self.a * rotation_matrix[1][0] + self.b * rotation_matrix[1][1];
        let new_c = self.c;

        // 返回新的直线方程
        LinearEquation {
            a: new_a,
            b: new_b,
            c: new_c,
        }
    }

    // 将直线绕任意点逆时针旋转 theta 弧度，返回新的直线方程
    pub fn rotate_around_point(&self, theta: f64, center: (f64, f64)) -> LinearEquation {
        // 计算旋转矩阵
        let cos_theta = theta.cos();
        let sin_theta = theta.sin();

        // 将直线平移到旋转中心
        let mut translated_line = self.translate(-center.0, -center.1);

        // 应用旋转矩阵
        let new_a = self.a * cos_theta - self.b * sin_theta;
        let new_b = self.a * sin_theta + self.b * cos_theta;

        // 更新新的系数
        translated_line.a = new_a;
        translated_line.b = new_b;

        // 将直线还原到原来的位置
        translated_line.translate(center.0, center.1)
    }

    // 将直线沿 x 轴平移 h 个单位，沿 y 轴平移 k 个单位
    pub fn translate(&self, h: f64, k: f64) -> LinearEquation {
        LinearEquation {
            a: self.a,
            b: self.b,
            c: self.c + self.a * h + self.b * k,
        }
    }
    // 计算直线与 X 轴和 Y 轴的夹角（弧度）
    pub fn angles_with_axes(&self) -> (f64, f64) {
        // 计算斜率
        let slope = -self.a / self.b;

        // 计算与 X 轴的夹角
        let angle_with_x_axis = slope.atan();

        // 计算与 Y 轴的夹角
        let angle_with_y_axis = PI / 2.0 - angle_with_x_axis;

        (angle_with_x_axis, angle_with_y_axis)
    }
    // 判断点与直线的位置关系
    pub fn point_line_relationship(&self, point: &Point2D) -> PointLineRelationship {
        let result = self.a * point.x + self.b * point.y + self.c;

        if result == 0.0 {
            PointLineRelationship::OnLine
        } else if result > 0.0 {
            PointLineRelationship::AboveLine
        } else {
            PointLineRelationship::BelowLine
        }
    }

    // 判断直线与圆是否相切
    pub fn is_tangent_to_circle(&self, circle: &Circle) -> bool {
        // 计算直线到圆心的距离
        let distance_to_center = (self.a * circle.x + self.b * circle.y + self.c).abs()
            / f64::sqrt(self.a.powi(2) + self.b.powi(2));

        // 判断是否相切（距离差小于 EPSILON，考虑浮点数误差）
        (distance_to_center - circle.radius).abs() < EPSILON
    }
    // 判断直线是否垂直于 X 轴
    pub fn is_vertical_to_x_axis(&self) -> bool {
        self.b == 0.0
    }

    // 判断直线是否垂直于 Y 轴
    pub fn is_vertical_to_y_axis(&self) -> bool {
        self.a == 0.0
    }

    // 判断两条直线是否相交
    pub fn are_intersecting(&self, other: &LinearEquation) -> bool {
        !(self.is_parallel_to(other) || self.is_equal_to(other))
    }

    // 判断两条直线是否平行
    pub fn are_parallel(&self, other: &LinearEquation) -> bool {
        self.a * other.b == self.b * other.a
    }

    // 判断两条直线是否垂直
    pub fn are_perpendicular(&self, other: &LinearEquation) -> bool {
        self.a * other.a + self.b * other.b == 0.0
    }

    // 判断两条直线是否相等
    pub fn is_equal_to(&self, other: &LinearEquation) -> bool {
        self.a == other.a && self.b == other.b && self.c == other.c
    }

    // 获取直线的斜率
    pub fn slope(&self) -> Option<f64> {
        if self.is_vertical_to_x_axis() {
            None // 斜率不存在
        } else {
            Some(-self.a / self.b)
        }
    }
    /// 将一般式直线方程转换为斜率截距形式。
    ///
    /// # Returns
    ///
    /// 如果直线方程的 B 不为零，则返回 `Some((slope, intercept))`，
    /// 其中 `slope` 是斜率，`intercept` 是截距。
    ///
    /// 如果直线方程的 B 为零（垂直于 X 轴），则返回 `None`，因为斜率不存在。
    pub fn to_slope_intercept_form(&self) -> Option<(f64, f64)> {
        if self.b != 0.0 {
            let slope = -self.a / self.b;
            let intercept = -self.c / self.b;
            Some((slope, intercept))
        } else {
            None // 如果 B 为零，斜率不存在
        }
    }

    /// 将一般式直线方程转换为点斜式。
    ///
    /// # Returns
    ///
    /// 如果直线方程的 B 不为零，则返回 `Some((slope, point))`，
    /// 其中 `slope` 是斜率，`point` 是直线上的一点 (x, y)。
    ///
    /// 如果直线方程的 B 为零（垂直于 X 轴），则返回 `None`，因为斜率不存在。
    pub fn to_point_slope_form(&self) -> Option<(f64, (f64, f64))> {
        if self.b != 0.0 {
            let slope = -self.a / self.b;
            let point = (0.0, -self.c / self.b);
            Some((slope, point))
        } else {
            None // 如果 B 为零，斜率不存在
        }
    }
}