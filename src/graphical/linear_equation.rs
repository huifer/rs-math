use std::f64::consts::PI;
use std::f64::EPSILON;
use std::fmt::Debug;
use crate::graphical::circle::Circle;
use crate::graphical::point_2d::Point2D;

/// 代表二维平面上的直线方程。
#[derive(Debug)]
pub struct LinearEquation {
    /// 直线方程的 x 系数。
    pub a: f64,
    /// 直线方程的 y 系数。
    pub b: f64,
    /// 直线方程的常数项。
    pub c: f64,
}

/// 表示点与直线之间的关系。
#[derive(Debug, PartialEq, Clone)]
pub enum PointLineRelationship {
    /// 点在直线上。
    OnLine,
    /// 点在直线上方。
    AboveLine,
    /// 点在直线下方。
    BelowLine,
}
#[allow(dead_code)]
impl LinearEquation {
    /// 将直线方程转换为字符串表示。
    pub fn to_string(&self) -> String {
        format!("{}x + {}y + {} = 0", self.a, self.b, self.c)
    }
    /// 通过两点计算切线方程的一般式表示。
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
    /// 返回切线方程的一般式表示。
    ///
    /// # 示例
    ///
    /// ```
    /// use rs_math::graphical::linear_equation::LinearEquation;
    /// let equation = LinearEquation::from_points(1.0, 2.0, 3.0, 4.0);
    /// ```
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
    /// 从点斜式参数创建一般式方程。
    ///
    /// # 参数
    ///
    /// * `x1` - 直线上的一点的 x 坐标。
    /// * `y1` - 直线上的一点的 y 坐标。
    /// * `slope` - 直线的斜率。
    ///
    /// # 返回值
    ///
    /// 返回一般式方程的实例。
    ///
    /// # 示例
    ///
    /// ```
    /// use rs_math::graphical::linear_equation::LinearEquation;
    /// let equation = LinearEquation::from_point_slope(1.0, 2.0, 3.0);
    /// ```
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
    /// 检查两条直线是否平行。
    ///
    /// # 参数
    ///
    /// * `other` - 与当前直线比较的另一条直线。
    ///
    /// # 返回值
    ///
    /// 如果两条直线平行，返回 `true`，否则返回 `false`。
    ///
    /// # 示例
    ///
    /// ```
    /// use rs_math::graphical::linear_equation::LinearEquation;
    /// let line1 = LinearEquation { a: 2.0, b: 3.0, c: 4.0 };
    /// let line2 = LinearEquation { a: 2.0, b: 3.0, c: 7.0 };
    /// assert!(line1.is_parallel_to(&line2));
    /// ```
    pub fn is_parallel_to(&self, other: &LinearEquation) -> bool {
        // 两条直线平行的条件是它们的斜率相等
        if let (Some(slope1), Some(slope2)) = (self.slope(), other.slope()) {
            slope1 == slope2
        } else {
            false // 如果其中一条直线的斜率无法计算，则无法判断是否平行
        }
    }

    /// 从截距式方程创建一般式方程。
    ///
    /// # 参数
    ///
    /// * `m` - 直线的斜率。
    /// * `b` - y 轴截距。
    ///
    /// # 返回值
    ///
    /// 返回包含一般式方程的 `LinearEquation` 结构体实例。
    ///
    /// # 示例
    ///
    /// ```
    /// use rs_math::graphical::linear_equation::LinearEquation;
    /// let line = LinearEquation::from_slope_intercept(2.0, 3.0);
    /// assert_eq!(line.to_string(), "-2x + y - 3 = 0");
    /// ```
    pub fn from_slope_intercept(m: f64, b: f64) -> Self {
        // 一般式方程的 A 系数为 -m
        let a = -m;
        // 一般式方程的 B 系数为 1
        // 一般式方程的 C 系数为 -b
        let c = -b;

        // 创建并返回一般式方程的实例
        LinearEquation { a, b: 1.0, c }
    }

    /// 通过圆弧的两个端点计算切线方程的一般式表示。
    ///
    /// # 参数
    ///
    /// * `radius` - 圆弧的半径。
    /// * `x1` - 第一个端点的 x 坐标。
    /// * `y1` - 第一个端点的 y 坐标。
    /// * `x2` - 第二个端点的 x 坐标。
    /// * `y2` - 第二个端点的 y 坐标。
    ///
    /// # 返回值
    ///
    /// 返回包含切线方程的一般式表示的 `LinearEquation` 结构体实例。
    ///
    /// # 示例
    ///
    /// ```
    /// use rs_math::graphical::linear_equation::LinearEquation;
    /// let line = LinearEquation::from_arc(1.0, 0.0, 0.0, 1.0, 0.0);
    /// assert_eq!(line.to_string(), "0x + y - 1 = 0");
    /// ```
    /// todo: 是否需要增加两点+半径求圆弧，再求切线方程
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

    /// 将直线沿 x 轴平移指定单位，返回新的直线方程。
    ///
    /// # 参数
    ///
    /// * `h` - 沿 x 轴的平移距离。
    ///
    /// # 返回值
    ///
    /// 返回包含平移后直线方程的一般式表示的 `LinearEquation` 结构体实例。
    ///
    /// # 示例
    ///
    /// ```
    /// use rs_math::graphical::linear_equation::LinearEquation;
    /// let line = LinearEquation { a: 1.0, b: 2.0, c: 3.0 };
    /// let translated_line = line.translate_along_x(2.0);
    /// assert_eq!(translated_line.to_string(), "1x + 2y - 1 = 0");
    /// ```
    pub fn translate_along_x(&self, h: f64) -> LinearEquation {
        LinearEquation {
            a: self.a,
            b: self.b,
            c: self.c + h,
        }
    }

    /// 将直线绕原点逆时针旋转指定弧度，返回新的直线方程。
    ///
    /// # 参数
    ///
    /// * `theta` - 逆时针旋转的弧度。
    ///
    /// # 返回值
    ///
    /// 返回包含旋转后直线方程的一般式表示的 `LinearEquation` 结构体实例。
    ///
    /// # 示例
    ///
    /// ```
    /// use rs_math::graphical::linear_equation::LinearEquation;
    /// let line = LinearEquation { a: 1.0, b: 2.0, c: 3.0 };
    /// let rotated_line = line.rotate_around_origin(1.0);
    /// // 根据旋转后的具体结果进行验证
    /// ```
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

    /// 将直线绕任意点逆时针旋转指定弧度，返回新的直线方程。
    ///
    /// # 参数
    ///
    /// * `theta` - 逆时针旋转的弧度。
    /// * `center` - 旋转中心的坐标。
    ///
    /// # 返回值
    ///
    /// 返回包含旋转后直线方程的一般式表示的 `LinearEquation` 结构体实例。
    ///
    /// # 示例
    ///
    /// ```
    /// use rs_math::graphical::linear_equation::LinearEquation;
    /// let line = LinearEquation { a: 1.0, b: 2.0, c: 3.0 };
    /// let rotated_line = line.rotate_around_point(1.0, (0.0, 0.0));
    /// // 根据旋转后的具体结果进行验证
    /// ```
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

    /// 将直线沿 x 轴平移指定距离，沿 y 轴平移指定距离。
    ///
    /// # 参数
    ///
    /// * `h` - 沿 x 轴的平移距离。
    /// * `k` - 沿 y 轴的平移距离。
    ///
    /// # 返回值
    ///
    /// 返回包含平移后直线方程的一般式表示的 `LinearEquation` 结构体实例。
    ///
    /// # 示例
    ///
    /// ```
    /// use rs_math::graphical::linear_equation::LinearEquation;
    /// let line = LinearEquation { a: 1.0, b: 2.0, c: 3.0 };
    /// let translated_line = line.translate(2.0, 3.0);
    /// ```
    pub fn translate(&self, h: f64, k: f64) -> LinearEquation {
        LinearEquation {
            a: self.a,
            b: self.b,
            c: self.c + self.a * h + self.b * k,
        }
    }
    /// 计算直线与 X 轴和 Y 轴的夹角（弧度）。
    ///
    /// # 返回值
    ///
    /// 返回包含直线与 X 轴和 Y 轴夹角的元组 `(angle_with_x_axis, angle_with_y_axis)`。
    ///
    /// # 示例
    ///
    /// ```
    /// use rs_math::graphical::linear_equation::LinearEquation;
    /// let line = LinearEquation { a: 1.0, b: 2.0, c: 3.0 };
    /// let (angle_with_x_axis, angle_with_y_axis) = line.angles_with_axes();
    /// 
    /// ```
    pub fn angles_with_axes(&self) -> (f64, f64) {
        // 计算斜率
        let slope = -self.a / self.b;

        // 计算与 X 轴的夹角
        let angle_with_x_axis = slope.atan();

        // 计算与 Y 轴的夹角
        let angle_with_y_axis = PI / 2.0 - angle_with_x_axis;

        (angle_with_x_axis, angle_with_y_axis)
    }
    /// 判断点与直线的位置关系。
    ///
    /// # 参数
    ///
    /// * `point` - 要判断的点的坐标。
    ///
    /// # 返回值
    ///
    /// 返回 [`PointLineRelationship`] 枚举，表示点与直线的位置关系。
    ///
    /// # 示例
    ///
    /// ```
    /// use rs_math::graphical::linear_equation::{LinearEquation, PointLineRelationship};
    /// use rs_math::graphical::point_2d::Point2D;
    ///
    /// let line = LinearEquation { a: 1.0, b: -1.0, c: 0.0 };
    /// let point = Point2D { x: 2.0, y: 2.0 };
    /// let relationship = line.point_line_relationship(&point);
    /// 
    /// ```
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

    /// 判断直线与圆是否相切。
    ///
    /// # 参数
    ///
    /// * `circle` - 圆的信息。
    ///
    /// # 返回值
    ///
    /// 如果直线与圆相切，返回 `true`；否则，返回 `false`。
    ///
    /// # 注意
    ///
    /// 在比较距离时，考虑到浮点数误差，使用了 EPSILON 常量。
    ///
    /// # 示例
    ///
    /// ```
    /// use rs_math::graphical::linear_equation::LinearEquation;
    /// use rs_math::graphical::circle::Circle;
    ///
    /// let line = LinearEquation { a: 1.0, b: -1.0, c: 0.0 };
    /// let circle = Circle::new(0.0, 0.0, 1.0);
    /// let is_tangent = line.is_tangent_to_circle(&circle);
    /// 
    /// ```
    pub fn is_tangent_to_circle(&self, circle: &Circle) -> bool {
        // 计算直线到圆心的距离
        let distance_to_center = (self.a * circle.x + self.b * circle.y + self.c).abs()
            / f64::sqrt(self.a.powi(2) + self.b.powi(2));

        // 判断是否相切（距离差小于 EPSILON，考虑浮点数误差）
        (distance_to_center - circle.radius).abs() < EPSILON
    }
    /// 判断直线是否垂直于 X 轴。
    ///
    /// # 返回值
    ///
    /// 如果直线垂直于 X 轴，返回 `true`；否则，返回 `false`。
    ///
    /// # 示例
    ///
    /// ```
    /// use rs_math::graphical::linear_equation::LinearEquation;
    ///
    /// let line = LinearEquation { a: 2.0, b: 0.0, c: 3.0 };
    /// let is_vertical = line.is_vertical_to_x_axis();
    /// 
    /// ```
    pub fn is_vertical_to_x_axis(&self) -> bool {
        self.b == 0.0
    }

    /// 判断直线是否垂直于 Y 轴。
    ///
    /// # 返回值
    ///
    /// 如果直线垂直于 Y 轴，返回 `true`；否则，返回 `false`。
    ///
    /// # 示例
    ///
    /// ```
    /// use rs_math::graphical::linear_equation::LinearEquation;
    ///
    /// let line = LinearEquation { a: 0.0, b: 3.0, c: 5.0 };
    /// let is_vertical = line.is_vertical_to_y_axis();
    /// 
    /// ```
    pub fn is_vertical_to_y_axis(&self) -> bool {
        self.a == 0.0
    }

    /// 判断两条直线是否相交。
    ///
    /// # 返回值
    ///
    /// 如果两条直线相交，返回 `true`；否则，返回 `false`。
    ///
    /// # 示例
    ///
    /// ```
    /// use rs_math::graphical::linear_equation::LinearEquation;
    ///
    /// let line1 = LinearEquation { a: 2.0, b: -3.0, c: 5.0 };
    /// let line2 = LinearEquation { a: -4.0, b: 6.0, c: 8.0 };
    /// let do_intersect = line1.are_intersecting(&line2);
    /// 
    /// ```
    pub fn are_intersecting(&self, other: &LinearEquation) -> bool {
        !(self.is_parallel_to(other) || self.is_equal_to(other))
    }

    /// 判断两条直线是否平行。
    ///
    /// # 返回值
    ///
    /// 如果两条直线平行，返回 `true`；否则，返回 `false`。
    ///
    /// # 示例
    ///
    /// ```
    /// use rs_math::graphical::linear_equation::LinearEquation;
    ///
    /// let line1 = LinearEquation { a: 2.0, b: -3.0, c: 5.0 };
    /// let line2 = LinearEquation { a: 4.0, b: -6.0, c: 10.0 };
    /// let are_parallel = line1.are_parallel(&line2);
    /// 
    /// ```
    pub fn are_parallel(&self, other: &LinearEquation) -> bool {
        self.a * other.b == self.b * other.a
    }

    /// 判断两条直线是否垂直。
    ///
    /// # 返回值
    ///
    /// 如果两条直线垂直，返回 `true`；否则，返回 `false`。
    ///
    /// # 示例
    ///
    /// ```
    /// use rs_math::graphical::linear_equation::LinearEquation;
    ///
    /// let line1 = LinearEquation { a: 2.0, b: -3.0, c: 5.0 };
    /// let line2 = LinearEquation { a: 3.0, b: 2.0, c: -4.0 };
    /// let are_perpendicular = line1.are_perpendicular(&line2);
    ///
    /// ```
    pub fn are_perpendicular(&self, other: &LinearEquation) -> bool {
        self.a * other.a + self.b * other.b == 0.0
    }

    /// 判断两条直线是否相等。
    ///
    /// # 返回值
    ///
    /// 如果两条直线相等，返回 `true`；否则，返回 `false`。
    ///
    /// # 示例
    ///
    /// ```
    /// use rs_math::graphical::linear_equation::LinearEquation;
    ///
    /// let line1 = LinearEquation { a: 2.0, b: -3.0, c: 5.0 };
    /// let line2 = LinearEquation { a: 2.0, b: -3.0, c: 5.0 };
    /// let are_equal = line1.is_equal_to(&line2);
    ///
    /// ```
    pub fn is_equal_to(&self, other: &LinearEquation) -> bool {
        self.a == other.a && self.b == other.b && self.c == other.c
    }

    /// 获取直线的斜率。
    ///
    /// # 返回值
    ///
    /// 如果直线与 X 轴垂直，返回 `None`；否则，返回直线的斜率 `Some(slope)`。
    ///
    /// # 示例
    ///
    /// ```
    /// use rs_math::graphical::linear_equation::LinearEquation;
    ///
    /// let line = LinearEquation { a: 2.0, b: -3.0, c: 5.0 };
    /// let slope = line.slope();
    ///
    /// ```
    pub fn slope(&self) -> Option<f64> {
        if self.is_vertical_to_x_axis() {
            None // 斜率不存在
        } else {
            Some(-self.a / self.b)
        }
    }
    /// 将一般式直线方程转换为斜率截距形式。
    ///
    /// # 返回值
    ///
    /// 如果直线方程的 B 不为零，则返回 `Some((slope, intercept))`，
    /// 其中 `slope` 是斜率，`intercept` 是截距。
    ///
    /// 如果直线方程的 B 为零（垂直于 X 轴），则返回 `None`，因为斜率不存在。
    ///
    /// # 示例
    ///
    /// ```
    /// use rs_math::graphical::linear_equation::LinearEquation;
    ///
    /// let line = LinearEquation { a: 2.0, b: -3.0, c: 5.0 };
    /// let result = line.to_slope_intercept_form();
    ///
    /// ```
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
    /// # 返回值
    ///
    /// 如果直线方程的 B 不为零，则返回 `Some((slope, point))`，
    /// 其中 `slope` 是斜率，`point` 是直线上的一点 (x, y)。
    ///
    /// 如果直线方程的 B 为零（垂直于 X 轴），则返回 `None`，因为斜率不存在。
    ///
    /// # 示例
    ///
    /// ```
    /// use rs_math::graphical::linear_equation::LinearEquation;
    ///
    /// let line = LinearEquation { a: 2.0, b: -3.0, c: 5.0 };
    /// let result = line.to_point_slope_form();
    ///
    /// ```
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