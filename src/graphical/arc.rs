use crate::graphical::point::Point;
use crate::graphical::tangent::LinearEquation;

#[derive(Debug, PartialEq)]
struct Arc {
    radius: f64,
    theta: f64, // 中心角，以弧度表示
}


impl Arc {
    // 构造函数
    pub fn new(radius: f64, theta: f64) -> Arc {
        Arc { radius, theta }
    }

    // 计算弧长的方法
    pub fn calculate_arc_length(&self) -> f64 {
        self.radius * self.theta
    }

    // 计算圆弧的长度
    pub fn arc_length(&self) -> f64 {
        self.radius * self.theta
    }
    /// 以弧长和半径求圆弧
    pub fn from_arc_length_and_radius(arc_length: f64, radius: f64) -> Arc {
        // 初始猜测中心角的值
        let mut theta_guess = arc_length / radius;

        // 牛顿迭代法求解方程 s = r * theta
        let epsilon = 1e-10; // 精度要求
        let mut theta_prev;

        loop {
            theta_prev = theta_guess;
            let f_theta = radius * theta_prev - arc_length;
            let f_prime_theta = radius; // 导数 f'(θ) = r
            theta_guess = theta_prev - f_theta / f_prime_theta;

            if (theta_guess - theta_prev).abs() < epsilon {
                break;
            }
        }

        Arc {
            radius,
            theta: theta_guess,
        }
    }

    /// 已知弦长和半径求圆弧
    pub fn from_chord_length_and_radius(chord_length: f64, radius: f64) -> Arc {
        // 计算中心角的值
        let theta = 2.0 * f64::asin(chord_length / (2.0 * radius));

        Arc {
            radius,
            theta,
        }
    }


    /// 已知圆弧的两个端点坐标求圆弧
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

        Arc { radius, theta }
    }

    /// 已知圆弧的面积和半径求圆弧
    pub fn from_area_and_radius(area: f64, radius: f64) -> Arc {
        // 计算弧长
        let arc_length = (area * 2.0 / radius).sqrt();

        // 计算中心角
        let theta = arc_length / radius;

        Arc { radius, theta }
    }
    /// 生成圆弧上的点
    pub fn generate_points(&self, num_points: usize) -> Vec<Point> {
        let mut points = Vec::with_capacity(num_points);

        for i in 0..num_points {
            let theta_increment = self.theta / (num_points as f64 - 1.0);
            let current_theta = i as f64 * theta_increment;
            let x = self.radius * current_theta.cos();
            let y = self.radius * current_theta.sin();
            points.push(Point { x, y });
        }

        points
    }

    /// 计算圆弧上一点的坐标
    pub fn point_on_arc(&self, angle: f64) -> (f64, f64) {
        let x = self.radius * self.theta.cos();
        let y = self.radius * self.theta.sin();
        (x, y)
    }

    /// 计算圆弧上的切线方程
    pub fn tangent_at_point(&self, angle: f64) -> LinearEquation {
        // 计算两个相邻点的坐标
        let (x1, y1) = self.point_on_arc(angle);
        let epsilon = 1e-8;
        let (x2, y2) = self.point_on_arc(angle + epsilon);

        // 计算切线方程的一般式表示
        let a = y2 - y1;
        let b = x1 - x2;
        let c = x2 * y1 - x1 * y2;

        LinearEquation { A: a, B: b, C: c }
    }
    // 计算法线方程
    pub fn normal_at_point(&self, angle: f64) -> LinearEquation {
        // 计算切线方程的斜率
        let tangent_slope = -(self.radius * self.theta.sin()) / (self.radius * self.theta.cos());

        // 计算法线斜率
        let normal_slope = -1.0 / tangent_slope;

        // 计算法线方程的常数
        let (x0, y0) = self.point_on_arc(angle);
        let normal_constant = y0 - normal_slope * x0;

        Normal {
            A: normal_slope,
            B: -1.0,
            C: normal_constant,
        }
    }
}
