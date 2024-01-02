/// 定义二维向量结构体。
///
/// 该结构体包含两个分量，分别表示向量的 x 和 y 坐标。
///
/// # 字段
///
/// * `x`：向量的 x 坐标。
/// * `y`：向量的 y 坐标。
///
/// # 示例
///
/// ```rust
/// use rs_math::vector::vector_2d::Vector2D;
///
/// let v = Vector2D { x: 1.0, y: 2.0 };
///
/// ```

pub struct Vector2D {
    pub x: f64,
    pub y: f64,
}

// 实现一个方法来创建新的二维向量
impl Vector2D {
    /// 创建新的二维向量。
    ///
    /// 该函数创建一个新的二维向量，其 x 坐标和 y 坐标分别为 `x` 和 `y`。
    ///
    /// # 参数
    ///
    /// * `x`：向量的 x 坐标。
    /// * `y`：向量的 y 坐标。
    ///
    /// # 返回值
    ///
    /// 返回新的二维向量。
    ///
    /// # 示例
    ///
    /// ```rust
    /// use rs_math::vector::vector_2d::Vector2D;
    ///
    /// let v = Vector2D::new(1.0, 2.0);
    ///
    /// ```
    pub fn new(x: f64, y: f64) -> Vector2D {
        Vector2D { x, y }
    }

    /// 计算二维向量的模（长度）。
    ///
    /// 该函数使用欧几里得范数计算二维向量的模（长度）。
    ///
    /// # 参数
    ///
    /// * `self`：二维向量。
    ///
    /// # 返回值
    ///
    /// 返回二维向量的模（长度）。
    ///
    /// # 示例
    ///
    /// ```rust
    /// use rs_math::vector::vector_2d::Vector2D;
    ///
    /// let v = Vector2D::new(1.0, 2.0);
    ///
    /// ```
    pub fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }


    /// 计算两个二维向量的和，返回一个新的向量。
    ///
    /// 该函数使用算术加法运算计算两个二维向量的和。
    ///
    /// # 参数
    ///
    /// * `self`：调用该方法的向量。
    /// * `other`：要与 `self` 相加的另一个向量。
    ///
    /// # 返回值
    ///
    /// 返回一个新的向量，表示 `self` 和 `other` 的和。
    ///
    /// # 示例
    ///
    /// ```rust
    /// use rs_math::vector::vector_2d::Vector2D;
    ///
    /// let v1 = Vector2D::new(1.0, 2.0);
    /// let v2 = Vector2D::new(3.0, 4.0);
    ///
    /// let v3 = v1.add(v2);
    ///
    /// ```
    pub fn add(self, other: Vector2D) -> Vector2D {
        Vector2D { x: self.x + other.x, y: self.y + other.y }
    }

    // 向量减法
    pub fn subtract(self, other: Vector2D) -> Vector2D {
        Vector2D { x: self.x - other.x, y: self.y - other.y }
    }

    // 向量点积
    pub fn dot_product(self, other: Vector2D) -> f64 {
        self.x * other.x + self.y * other.y
    }

    // 向量叉积
    pub fn cross_product(self, other: Vector2D) -> f64 {
        self.x * other.y - self.y * other.x
    }
}