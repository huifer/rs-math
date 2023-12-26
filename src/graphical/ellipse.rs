use std::f64::consts::PI;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Ellipse {
    h: f64,
    // 椭圆中心的 x 坐标
    k: f64,
    // 椭圆中心的 y 坐标
    a: f64,
    // 长轴的长度
    b: f64, // 短轴的长度
    phi: f64, // 旋转角度（可能为零）
}

pub enum PointPosition {
    OnEllipse,
    InsideEllipse,
    OutsideEllipse,
}

#[allow(dead_code)]
impl Ellipse {
    // 构造函数，用于创建新的椭圆实例
    pub  fn new(a: f64, b: f64, h: f64, k: f64, phi: f64) -> Ellipse {
        Ellipse { a, b, h, k, phi }
    }

    // 计算椭圆离心率的方法
    pub fn eccentricity(&self) -> f64 {
        (1.0 - self.b.powi(2) / self.a.powi(2)).sqrt()
    }
    // 输出椭圆的方程
    pub fn print_equation(&self) {
        println!("椭圆方程：(x - {})^2 / {}^2 + (y - {})^2 / {}^2 = 1", self.h, self.a, self.k, self.b);
    }

    // 计算椭圆的面积
    pub fn area(&self) -> f64 {
        PI * self.a * self.b
    }
    // 估算椭圆的周长
    pub fn estimate_circumference(&self) -> f64 {
        PI * (3.0 * (self.a + self.b) - ((3.0 * self.a + self.b) * (self.a + 3.0 * self.b)).sqrt())
    }
    // 判断点和椭圆的关系并返回枚举值
    pub fn point_position(&self, x: f64, y: f64) -> PointPosition {
        let distance_squared = ((x - self.h) / self.a).powi(2) + ((y - self.k) / self.b).powi(2);

        if distance_squared == 1.0 {
            PointPosition::OnEllipse
        } else if distance_squared < 1.0 {
            PointPosition::InsideEllipse
        } else {
            PointPosition::OutsideEllipse
        }
    }
    // 计算椭圆的焦点坐标
    pub fn calculate_foci(&self) -> ((f64, f64), (f64, f64)) {
        let c = (self.a.powi(2) - self.b.powi(2)).sqrt();
        let f1 = (self.h + c, self.k);
        let f2 = (self.h - c, self.k);

        (f1, f2)
    }
    // 拟合椭圆
    fn fit(points: &[(f64, f64)]) -> Option<Ellipse> {
        let n = points.len();
        if n < 5 {
            return None; // 至少需要 5 个点
        }

        // 归一化数据
        let (mean_x, mean_y) = points.iter().fold((0.0, 0.0), |acc, &(x, y)| (acc.0 + x, acc.1 + y));
        let mean_x = mean_x / n as f64;
        let mean_y = mean_y / n as f64;

        let normalized_points: Vec<(f64, f64)> = points
            .iter()
            .map(|&(x, y)| (x - mean_x, y - mean_y))
            .collect();

        // 构建设计矩阵
        let mut d_matrix: Vec<Vec<f64>> = Vec::with_capacity(n);
        for i in 0..n {
            let (x, y) = normalized_points[i];
            d_matrix.push(vec![x * x, x * y, y * y, x, y]);
        }

        // 构建响应矩阵
        let r_matrix: Vec<Vec<f64>> = vec![vec![1.0; 1]; n];

        // 解线性方程组
        let result = gauss_elimination(&d_matrix, &r_matrix);

        match result {
            Some(parameters) => {
                // 重建椭圆方程
                let a = parameters[0];
                let b = parameters[1];
                let c = parameters[2];
                let d = parameters[3];
                let e = parameters[4];

                // 反归一化
                let h = mean_x - (2.0 * c * d + b * e) / (4.0 * a * c - b.powi(2));
                let k = mean_y - (b * d + 2.0 * a * e) / (4.0 * a * c - b.powi(2));

                let phi = 0.5 * (c - a).atan2(b);

                Some(Ellipse::new(1.0 / a, 1.0 / c, h, k, phi))
            }
            None => None,
        }
    }
}

// 高斯消元法解线性方程组
fn gauss_elimination(a: &Vec<Vec<f64>>, b: &Vec<Vec<f64>>) -> Option<Vec<f64>> {
    let n = a.len();
    let m = a[0].len();

    let mut augmented_matrix = vec![vec![0.0; m + 1]; n];
    for i in 0..n {
        for j in 0..m {
            augmented_matrix[i][j] = a[i][j];
        }
        augmented_matrix[i][m] = b[i][0];
    }

    for i in 0..n {
        // 高斯消元
        for k in i + 1..n {
            let factor = augmented_matrix[k][i] / augmented_matrix[i][i];
            for j in i..m + 1 {
                augmented_matrix[k][j] -= factor * augmented_matrix[i][j];
            }
        }
    }

    // 回代
    let mut result = vec![0.0; n];
    for i in (0..n).rev() {
        result[i] = augmented_matrix[i][m];
        for j in i + 1..n {
            result[i] -= augmented_matrix[i][j] * result[j];
        }
        result[i] /= augmented_matrix[i][i];
    }

    Some(result)
}
