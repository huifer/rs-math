/// 二维空间中的点的结构体表示。
#[derive(Debug, PartialEq, Clone)]
pub struct Point2D {
    /// X 坐标
    pub x: f64,

    /// Y 坐标
    pub y: f64,
}

#[allow(dead_code)]
impl Point2D
{
    /// 计算两个点之间的欧几里德距离。
    ///
    /// # 参数
    ///
    /// - `point2`: 第二个点的坐标。
    ///
    /// # 返回值
    ///
    /// 两点之间的欧几里德距离。
    ///
    /// # 示例
    ///
    /// ```
    /// use rs_math::graphical::point_2d::Point2D;
    ///
    /// let point1 = Point2D { x: 1.0, y: 2.0 };
    /// let point2 = Point2D { x: 4.0, y: 6.0 };
    /// let distance = point1.distance_between_points(point2);
    /// // 验证距离是否符合预期
    /// ```
    pub fn distance_between_points(&self, point2: Point2D) -> f64 {
        let dx = point2.x - self.x;
        let dy = point2.y - self.y;
        (dx * dx + dy * dy).sqrt()
    }

    /// 绕原点逆时针旋转点。
    ///
    /// # 参数
    ///
    /// - `angle`: 旋转角度（弧度）。
    ///
    /// # 返回值
    ///
    /// 旋转后的点的坐标。
    ///
    /// # 示例
    ///
    /// ```
    /// use rs_math::graphical::point_2d::Point2D;
    ///
    /// let point = Point2D { x: 3.0, y: 4.0 };
    /// let rotated_point = point.rotate(std::f64::consts::FRAC_PI_2);
    /// // 验证旋转后的坐标是否符合预期
    /// ```
    pub fn rotate(&self, angle: f64) -> Point2D {
        let cos_theta = angle.cos();
        let sin_theta = angle.sin();

        let new_x = self.x * cos_theta - self.y * sin_theta;
        let new_y = self.x * sin_theta + self.y * cos_theta;

        Point2D { x: new_x, y: new_y }
    }

    /// 计算点到另一点的欧几里德距离。
    ///
    /// # 参数
    ///
    /// - `other`: 另一点的坐标。
    ///
    /// # 返回值
    ///
    /// 两点之间的欧几里德距离。
    ///
    /// # 示例
    ///
    /// ```
    /// use rs_math::graphical::point_2d::Point2D;
    ///
    /// let point1 = Point2D { x: 1.0, y: 2.0 };
    /// let point2 = Point2D { x: 4.0, y: 6.0 };
    /// let distance = point1.distance_to(&point2);
    ///
    /// ```
    pub fn distance_to(&self, other: &Point2D) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }

    /// 将两个点的坐标相加，得到新的点。
    ///
    /// # 参数
    ///
    /// - `p1`: 第一个点的坐标。
    /// - `p2`: 第二个点的坐标。
    ///
    /// # 返回值
    ///
    /// 两点坐标相加后得到的新点。
    ///
    /// # 示例
    ///
    /// ```
    /// use rs_math::graphical::point_2d::Point2D;
    ///
    /// let point1 = Point2D { x: 1.0, y: 2.0 };
    /// let point2 = Point2D { x: 3.0, y: 4.0 };
    /// let result = Point2D::add_points(&point1, &point2);
    ///
    /// ```
    pub fn add_points(p1: &Point2D, p2: &Point2D) -> Point2D {
        Point2D { x: p1.x + p2.x, y: p1.y + p2.y }
    }
}
