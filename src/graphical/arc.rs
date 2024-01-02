use crate::graphical::point_2d::Point2D;
use crate::graphical::linear_equation::LinearEquation;

/// 表示一个圆弧的结构体。
#[derive(Debug, PartialEq)]
pub struct Arc {
    /// 弧的半径。
    pub radius: f64,
    /// 中心角，以弧度表示。
    pub theta: f64,
}


#[allow(dead_code)]
impl Arc {
    /// 构造一个新的弧实例。
    ///
    /// # 参数
    ///
    /// * `radius` - 弧的半径。
    /// * `theta` - 中心角，以弧度表示。
    ///
    /// # 返回值
    ///
    /// 返回一个包含给定半径和中心角的 Arc 结构体实例。
    ///
    /// # 示例
    ///
    /// ```
    /// use rs_math::graphical::arc::Arc;
    /// let arc = Arc::new(5.0, 1.2);
    /// ```
    pub fn new(radius: f64, theta: f64) -> Arc {
        Arc { radius, theta }
    }

    /// 计算弧的长度。
    ///
    /// # 返回值
    ///
    /// 返回弧的长度，通过半径和中心角计算得出。
    ///
    /// # 示例
    ///
    /// ```
    /// use rs_math::graphical::arc::Arc;
    /// let arc = Arc::new(5.0, 1.2);
    /// let arc_length = arc.calculate_arc_length();
    /// ```
    pub fn calculate_arc_length(&self) -> f64 {
        self.radius * self.theta
    }

    /// 计算圆弧的长度。
    ///
    /// # 返回值
    ///
    /// 返回圆弧的长度，通过半径和中心角计算得出。
    ///
    /// # 示例
    ///
    /// ```
    /// use rs_math::graphical::arc::Arc;
    /// let arc = Arc::new(5.0, 1.2);
    /// let arc_length = arc.arc_length();
    /// ```
    pub fn arc_length(&self) -> f64 {
        // 返回圆弧的长度，即半径乘以中心角
        self.radius * self.theta
    }
    /// 根据弧长和半径计算圆弧的属性并返回一个新的 Arc 实例。
    ///
    /// # 参数
    ///
    /// * `arc_length` - 圆弧的长度。
    /// * `radius` - 圆弧的半径。
    ///
    /// # 返回值
    ///
    /// 返回一个包含给定弧长和半径计算得出的半径和中心角的 Arc 结构体实例。
    ///
    /// # 示例
    ///
    /// ```
    /// use rs_math::graphical::arc::Arc;
    /// let arc = Arc::from_arc_length_and_radius(10.0, 2.0);
    /// ```
    pub fn from_arc_length_and_radius(arc_length: f64, radius: f64) -> Arc {
        // 初始猜测中心角的值
        let mut theta_guess = arc_length / radius;

        // 牛顿迭代法求解方程 s = r * theta
        let epsilon = 1e-10; // 精度要求
        let mut theta_prev;

        // 迭代直到满足精度要求
        loop {
            theta_prev = theta_guess;
            let f_theta = radius * theta_prev - arc_length;
            let f_prime_theta = radius; // 导数 f'(θ) = r
            theta_guess = theta_prev - f_theta / f_prime_theta;

            // 判断是否达到精度要求，是则退出循环
            if (theta_guess - theta_prev).abs() < epsilon {
                break;
            }
        }

        // 返回新的 Arc 结构体实例
        Arc {
            radius,
            theta: theta_guess,
        }
    }

    /// 根据弦长和半径计算圆弧的属性并返回一个新的 Arc 实例。
    ///
    /// # 参数
    ///
    /// * `chord_length` - 圆弧的弦长。
    /// * `radius` - 圆弧的半径。
    ///
    /// # 返回值
    ///
    /// 返回一个包含给定弦长和半径计算得出的半径和中心角的 Arc 结构体实例。
    ///
    /// # 示例
    ///
    /// ```
    /// use rs_math::graphical::arc::Arc;
    /// let arc = Arc::from_chord_length_and_radius(6.0, 3.0);
    /// ```
    pub fn from_chord_length_and_radius(chord_length: f64, radius: f64) -> Arc {
        // 计算中心角的值
        let theta = 2.0 * f64::asin(chord_length / (2.0 * radius));

        // 返回新的 Arc 结构体实例
        Arc {
            radius,
            theta,
        }
    }


    /// 根据圆弧的两个端点坐标计算圆弧的属性并返回一个新的 Arc 实例。
    ///
    /// # 参数
    ///
    /// * `x1` - 第一个端点的 x 坐标。
    /// * `y1` - 第一个端点的 y 坐标。
    /// * `x2` - 第二个端点的 x 坐标。
    /// * `y2` - 第二个端点的 y 坐标。
    ///
    /// # 返回值
    ///
    /// 返回一个包含给定两个端点坐标计算得出的半径和中心角的 Arc 结构体实例。
    ///
    /// # 示例
    ///
    /// ```
    /// use rs_math::graphical::arc::Arc;
    /// let arc = Arc::from_endpoints(0.0, 0.0, 1.0, 0.0);
    /// ```
    pub fn from_endpoints(x1: f64, y1: f64, x2: f64, y2: f64) -> Arc {
        // 计算半径
        let radius = ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt();

        // 计算圆心坐标
        let center_x = (x1 + x2) / 2.0;
        let center_y = (y1 + y2) / 2.0;

        // 计算中心角
        let v1_x = x1 - center_x;
        let v1_y = y1 - center_y;
        let v2_x = x2 - center_x;
        let v2_y = y2 - center_y;

        let dot_product = v1_x * v2_x + v1_y * v2_y;
        let magnitude_product = (v1_x.powi(2) + v1_y.powi(2)).sqrt() * (v2_x.powi(2) + v2_y.powi(2)).sqrt();

        let cos_theta = dot_product / magnitude_product;
        let theta = cos_theta.acos();

        // 返回新的 Arc 结构体实例
        Arc { radius, theta }
    }

    /// 根据圆弧的面积和半径计算圆弧的属性并返回一个新的 Arc 实例。
    ///
    /// # 参数
    ///
    /// * `area` - 圆弧的面积。
    /// * `radius` - 圆弧的半径。
    ///
    /// # 返回值
    ///
    /// 返回一个包含给定面积和半径计算得出的半径和中心角的 Arc 结构体实例。
    ///
    /// # 示例
    ///
    /// ```
    /// use rs_math::graphical::arc::Arc;
    /// let arc = Arc::from_area_and_radius(10.0, 2.0);
    /// ```
    pub fn from_area_and_radius(area: f64, radius: f64) -> Arc {
        // 计算弧长
        let arc_length = (area * 2.0 / radius).sqrt();

        // 计算中心角
        let theta = arc_length / radius;

        // 返回新的 Arc 结构体实例
        Arc { radius, theta }
    }
    /// 生成圆弧上的点坐标并返回一个包含这些点的 Vector。
    ///
    /// # 参数
    ///
    /// * `num_points` - 生成的点的数量。
    ///
    /// # 返回值
    ///
    /// 返回一个包含生成的点坐标的 Vector。
    ///
    /// # 示例
    ///
    /// ```
    /// use rs_math::graphical::arc::Arc;
    /// use rs_math::geometry::point::Point2D;
    ///
    /// let arc = Arc::new(5.0, 1.2);
    /// let points = arc.generate_points(10);
    /// ```
    pub fn generate_points(&self, num_points: usize) -> Vec<Point2D> {
        // 创建一个 Vector 以存储生成的点
        let mut points = Vec::with_capacity(num_points);

        // 循环生成点坐标
        for i in 0..num_points {
            let theta_increment = self.theta / (num_points as f64 - 1.0);
            let current_theta = i as f64 * theta_increment;
            let x = self.radius * current_theta.cos();
            let y = self.radius * current_theta.sin();

            // 将点坐标添加到 Vector 中
            points.push(Point2D { x, y });
        }

        // 返回包含生成的点坐标的 Vector
        points
    }
    /// 计算圆弧上给定角度处的点坐标。
    ///
    /// # 参数
    ///
    /// * `angle` - 圆弧上的角度，以弧度表示。
    ///
    /// # 返回值
    ///
    /// 返回一个包含给定角度处的点坐标的元组。
    ///
    /// # 示例
    ///
    /// ```
    /// use rs_math::graphical::arc::Arc;
    ///
    /// let arc = Arc::new(5.0, 1.2);
    /// let point = arc.point_on_arc(0.5);
    /// ```
    pub fn point_on_arc(&self, angle: f64) -> (f64, f64) {
        // 计算圆弧上给定角度处的点坐标
        let x = self.radius * angle.cos();
        let y = self.radius * angle.sin();

        // 返回点坐标的元组
        (x, y)
    }

    /// 计算圆弧上给定角度处的切线方程。
    ///
    /// # 参数
    ///
    /// * `angle` - 圆弧上的角度，以弧度表示。
    ///
    /// # 返回值
    ///
    /// 返回一个包含切线方程的一般式表示的 LinearEquation 实例。
    ///
    /// # 示例
    ///
    /// ```
    /// use rs_math::graphical::arc::Arc;
    /// use rs_math::geometry::linear_equation::LinearEquation;
    ///
    /// let arc = Arc::new(5.0, 1.2);
    /// let tangent_equation = arc.tangent_at_point(0.5);
    /// ```
    pub fn tangent_at_point(&self, angle: f64) -> LinearEquation {
        // 计算两个相邻点的坐标
        let (x1, y1) = self.point_on_arc(angle);
        let epsilon = 1e-8;
        let (x2, y2) = self.point_on_arc(angle + epsilon);

        // 计算切线方程的一般式表示
        let a = y2 - y1;
        let b = x1 - x2;
        let c = x2 * y1 - x1 * y2;

        // 返回切线方程的 LinearEquation 实例
        LinearEquation { a, b, c }
    }
    /// 计算圆弧上给定角度处的法线方程。
    ///
    /// # 参数
    ///
    /// * `angle` - 圆弧上的角度，以弧度表示。
    ///
    /// # 返回值
    ///
    /// 返回一个包含法线方程的一般式表示的 LinearEquation 实例。
    ///
    /// # 示例
    ///
    /// ```
    /// use rs_math::graphical::arc::Arc;
    /// use rs_math::geometry::linear_equation::LinearEquation;
    ///
    /// let arc = Arc::new(5.0, 1.2);
    /// let normal_equation = arc.normal_at_point(0.5);
    /// ```
    pub fn normal_at_point(&self, angle: f64) -> LinearEquation {
        // 计算切线方程的斜率
        let tangent_slope = -(self.radius * self.theta.sin()) / (self.radius * self.theta.cos());

        // 计算法线斜率
        let normal_slope = -1.0 / tangent_slope;

        // 计算法线方程的常数
        let (x0, y0) = self.point_on_arc(angle);
        let normal_constant = y0 - normal_slope * x0;

        // 返回法线方程的 LinearEquation 实例
        LinearEquation {
            a: normal_slope,
            b: -1.0,
            c: normal_constant,
        }
    }
}
