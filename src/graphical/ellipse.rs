use std::f64::consts::PI;

/// 表示椭圆的结构体。
///
/// # 字段
///
/// * `h` - 椭圆中心的 x 坐标。
/// * `k` - 椭圆中心的 y 坐标。
/// * `a` - 长轴的长度。
/// * `b` - 短轴的长度。
/// * `phi` - 旋转角度（可能为零）。
///
/// 椭圆是一个二维图形，定义了平面上所有满足特定数学条件的点的集合。
/// 椭圆可以通过其中心坐标、长轴和短轴的长度以及旋转角度来描述。
/// 长轴是通过椭圆中心的两个焦点的距离，短轴是垂直于长轴的轴的长度。
/// 旋转角度表示椭圆相对于坐标轴的旋转程度。
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Ellipse {
    h: f64,
    // 椭圆中心的 x 坐标。
    k: f64,
    // 椭圆中心的 y 坐标。
    a: f64,
    // 长轴的长度。
    b: f64,
    // 短轴的长度。
    phi: f64,  // 旋转角度（可能为零）。
}

/// 表示点相对于椭圆的位置的枚举。
///
/// # 变体
///
/// * `OnEllipse` - 点在椭圆上。
/// * `InsideEllipse` - 点在椭圆内部。
/// * `OutsideEllipse` - 点在椭圆外部。
pub enum PointPosition {
    OnEllipse,
    InsideEllipse,
    OutsideEllipse,
}

#[allow(dead_code)]
impl Ellipse {
    /// 构造函数，用于创建新的椭圆实例。
    ///
    /// # 参数
    ///
    /// * `a` - 长轴的长度。
    /// * `b` - 短轴的长度。
    /// * `h` - 椭圆中心的 x 坐标。
    /// * `k` - 椭圆中心的 y 坐标。
    /// * `phi` - 旋转角度（可能为零）。
    ///
    /// # 返回值
    ///
    /// 返回一个包含给定参数的椭圆实例。
    ///
    /// # 示例
    ///
    /// ```
    /// use rs_math::graphical::ellipse::Ellipse;
    /// let ellipse = Ellipse::new(5.0, 3.0, 0.0, 0.0, 45.0);
    /// ```
    pub fn new(a: f64, b: f64, h: f64, k: f64, phi: f64) -> Ellipse {
        Ellipse { a, b, h, k, phi }
    }

    /// 计算椭圆离心率的方法。
    ///
    /// # 返回值
    ///
    /// 返回椭圆的离心率，计算公式为 sqrt(1 - (b^2 / a^2))。
    ///
    /// # 示例
    ///
    /// ```
    /// use rs_math::graphical::ellipse::Ellipse;
    /// let ellipse = Ellipse::new(5.0, 3.0, 0.0, 0.0, 45.0);
    /// let eccentricity = ellipse.eccentricity();
    /// ```
    pub fn eccentricity(&self) -> f64 {
        (1.0 - self.b.powi(2) / self.a.powi(2)).sqrt()
    }
    /// 输出椭圆的方程。
    ///
    /// 打印椭圆的标准方程，其中 `(x - h)^2 / a^2 + (y - k)^2 / b^2 = 1`。
    ///
    /// # 示例
    ///
    /// ```
    /// use rs_math::graphical::ellipse::Ellipse;
    /// let ellipse = Ellipse::new(5.0, 3.0, 0.0, 0.0, 45.0);
    /// ellipse.print_equation();
    /// ```
    pub fn print_equation(&self) {
        println!("椭圆方程：(x - {})^2 / {}^2 + (y - {})^2 / {}^2 = 1", self.h, self.a, self.k, self.b);
    }

    /// 计算椭圆的面积。
    ///
    /// 使用椭圆的长轴和短轴计算椭圆的面积，公式为 πab，其中 a 为长轴长度，b 为短轴长度。
    ///
    /// # 示例
    ///
    /// ```
    /// use rs_math::graphical::ellipse::Ellipse;
    /// let ellipse = Ellipse::new(5.0, 3.0, 0.0, 0.0, 0.0);
    /// let area = ellipse.area();
    /// println!("椭圆面积：{}", area);
    /// ```
    pub fn area(&self) -> f64 {
        PI * self.a * self.b
    }
    /// 估算椭圆的周长。
    ///
    /// 使用椭圆的长轴和短轴估算椭圆的周长。这是一个近似值，使用 Ramanujan 的公式：
    /// π * [3(a + b) - sqrt((3a + b)(a + 3b))]
    ///
    /// # 示例
    ///
    /// ```
    /// use rs_math::graphical::ellipse::Ellipse;
    /// let ellipse = Ellipse::new(5.0, 3.0, 0.0, 0.0, 0.0);
    /// let circumference = ellipse.estimate_circumference();
    /// println!("椭圆周长的估算值：{}", circumference);
    /// ```
    pub fn estimate_circumference(&self) -> f64 {
        PI * (3.0 * (self.a + self.b) - ((3.0 * self.a + self.b) * (self.a + 3.0 * self.b)).sqrt())
    }
    /// 判断点和椭圆的关系并返回枚举值。
    ///
    /// # 参数
    ///
    /// * `x` - 点的 x 坐标。
    /// * `y` - 点的 y 坐标。
    ///
    /// # 返回值
    ///
    /// 返回 `PointPosition` 枚举值，表示点和椭圆的关系。
    ///
    /// # 示例
    ///
    /// ```
    /// use rs_math::graphical::ellipse::{Ellipse, PointPosition};
    /// let ellipse = Ellipse::new(5.0, 3.0, 0.0, 0.0, 0.0);
    /// let position = ellipse.point_position(2.0, 1.0);
    /// match position {
    ///     PointPosition::OnEllipse => println!("点在椭圆上"),
    ///     PointPosition::InsideEllipse => println!("点在椭圆内部"),
    ///     PointPosition::OutsideEllipse => println!("点在椭圆外部"),
    /// }
    /// ```
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
    /// 计算椭圆的焦点坐标。
    ///
    /// # 返回值
    ///
    /// 返回包含两个焦点坐标的元组。
    ///
    /// # 示例
    ///
    /// ```
    /// use rs_math::graphical::ellipse::Ellipse;
    /// let ellipse = Ellipse::new(5.0, 3.0, 0.0, 0.0, 0.0);
    /// let foci_coordinates = ellipse.calculate_foci();
    /// println!("焦点1坐标: {:?}", foci_coordinates.0);
    /// println!("焦点2坐标: {:?}", foci_coordinates.1);
    /// ```
    pub fn calculate_foci(&self) -> ((f64, f64), (f64, f64)) {
        let c = (self.a.powi(2) - self.b.powi(2)).sqrt();
        let f1 = (self.h + c, self.k);
        let f2 = (self.h - c, self.k);

        (f1, f2)
    }
    /// 通过拟合给定点集合来估算椭圆。
    ///
    /// # 参数
    ///
    /// * `points` - 包含待拟合椭圆的点集合。
    ///
    /// # 返回值
    ///
    /// 如果拟合成功，返回一个包含椭圆参数的 `Option<Ellipse>`；否则返回 None。
    ///
    /// # 注意
    ///
    /// 至少需要 5 个点进行椭圆拟合。
    ///
    /// # 示例
    ///
    /// ```
    /// use rs_math::graphical::ellipse::Ellipse;
    ///
    /// let points = vec![(1.0, 0.0), (0.0, 1.0), (-1.0, 0.0), (0.0, -1.0), (0.0, 0.0)];
    /// let fitted_ellipse = Ellipse::fit(&points);
    ///
    /// match fitted_ellipse {
    ///     Some(ellipse) => {
    ///         println!("拟合椭圆成功！");
    ///         ellipse.print_equation();
    ///     }
    ///     None => println!("拟合椭圆失败，至少需要 5 个点。"),
    /// }
    /// ```
    pub fn fit(points: &[(f64, f64)]) -> Option<Ellipse> {
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

/// 使用高斯消元法解线性方程组。
///
/// # 参数
///
/// * `a` - 系数矩阵。
/// * `b` - 右侧常数矩阵。
///
/// # 返回值
///
/// 如果方程组有唯一解，返回解的向量；否则返回 None。
///
/// # 注意
///
/// - `a` 和 `b` 的行数应相同。
/// - 该函数会修改输入的矩阵。
///
/// # 示例
///
/// ```
/// let a = vec![vec![2.0, -1.0, 1.0], vec![-3.0, 2.0, -1.0], vec![-2.0, 1.0, 2.0]];
/// let b = vec![vec![8.0], vec![-11.0], vec![-3.0]];
///
/// match gauss_elimination(&a, &b) {
///     Some(result) => println!("解向量：{:?}", result),
///     None => println!("方程组无解或有无穷多解。"),
/// }
/// ```
pub fn gauss_elimination(a: &Vec<Vec<f64>>, b: &Vec<Vec<f64>>) -> Option<Vec<f64>> {
    let n = a.len();
    let m = a[0].len();

    // 构建增广矩阵
    let mut augmented_matrix = vec![vec![0.0; m + 1]; n];
    for i in 0..n {
        for j in 0..m {
            augmented_matrix[i][j] = a[i][j];
        }
        augmented_matrix[i][m] = b[i][0];
    }

    // 高斯消元
    for i in 0..n {
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
