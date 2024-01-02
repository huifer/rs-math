#[derive(Debug)]
pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3D {
    /// 向量加法操作。
    ///
    /// # 参数
    ///
    /// - `other`: 另一个向量。
    ///
    /// # 返回值
    ///
    /// 返回两个向量相加的结果，结果是一个新的 `Vector3D` 实例。
    ///
    /// # 示例
    ///
    /// ```
    /// use rs_math::vector::vector_3d::Vector3D;
    ///
    /// let vector1 = Vector3D { x: 1.0, y: 2.0, z: 3.0 };
    /// let vector2 = Vector3D { x: 4.0, y: 5.0, z: 6.0 };
    /// let result = vector1.add(&vector2);
    /// ```
    pub fn add(&self, other: &Vector3D) -> Vector3D {
        Vector3D {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    /// 向量减法操作。
    ///
    /// # 参数
    ///
    /// - `other`: 被减的向量。
    ///
    /// # 返回值
    ///
    /// 返回两个向量相减的结果，结果是一个新的 `Vector3D` 实例。
    ///
    /// # 示例
    ///
    /// ```
    /// use rs_math::vector::vector_3d::Vector3D;
    ///
    /// let vector1 = Vector3D { x: 1.0, y: 2.0, z: 3.0 };
    /// let vector2 = Vector3D { x: 4.0, y: 5.0, z: 6.0 };
    /// let result = vector1.subtract(&vector2);
    /// ```
    pub fn subtract(&self, other: &Vector3D) -> Vector3D {
        Vector3D {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    /// 数量乘法操作。
    ///
    /// # 参数
    ///
    /// - `scalar`: 乘以向量的标量。
    ///
    /// # 返回值
    ///
    /// 返回数量乘法的结果，结果是一个新的 `Vector3D` 实例。
    ///
    /// # 示例
    ///
    /// ```
    /// use rs_math::vector::vector_3d::Vector3D;
    ///
    /// let vector = Vector3D { x: 1.0, y: 2.0, z: 3.0 };
    /// let scalar = 2.0;
    /// let result = vector.scalar_multiply(scalar);
    /// ```
    pub fn scalar_multiply(&self, scalar: f64) -> Vector3D {
        Vector3D {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }

    /// 计算两个向量的叉积。
    ///
    /// # 参数
    ///
    /// - `other`: 另一个向量。
    ///
    /// # 返回值
    ///
    /// 返回两个向量的叉积，结果是一个新的 `Vector3D` 实例。
    ///
    /// # 示例
    ///
    /// ```
    /// use rs_math::vector::vector_3d::Vector3D;
    ///
    /// let vector1 = Vector3D { x: 1.0, y: 2.0, z: 3.0 };
    /// let vector2 = Vector3D { x: 4.0, y: 5.0, z: 6.0 };
    /// let cross_product = vector1.cross_product(&vector2);
    /// ```
    pub fn cross_product(&self, other: &Vector3D) -> Vector3D {
        Vector3D {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    /// 计算两个向量的点积。
    ///
    /// # 参数
    ///
    /// - `other`: 另一个向量。
    ///
    /// # 返回值
    ///
    /// 返回两个向量的点积。
    ///
    /// # 示例
    ///
    /// ```
    /// use rs_math::vector::vector_3d::Vector3D;
    ///
    /// let vector1 = Vector3D { x: 1.0, y: 2.0, z: 3.0 };
    /// let vector2 = Vector3D { x: 4.0, y: 5.0, z: 6.0 };
    /// let dot_product = vector1.dot_product(&vector2);
    /// ```
    pub fn dot_product(&self, other: &Vector3D) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    /// 计算向量的模长。
    ///
    /// # 返回值
    ///
    /// 返回向量的模长。
    ///
    /// # 示例
    ///
    /// ```
    /// use rs_math::vector::vector_3d::Vector3D;
    ///
    /// let vector = Vector3D { x: 3.0, y: 4.0, z: 5.0 };
    /// let magnitude = vector.magnitude();
    /// ```
    pub fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }
    /// 计算两个向量之间的夹角（弧度）。
    ///
    /// # 参数
    ///
    /// - `other`：要计算夹角的另一个向量。
    ///
    /// # 返回值
    ///
    /// 返回两个向量之间的夹角（弧度）。
    ///
    /// # 示例
    ///
    /// ```
    /// use rs_math::vector::vector_3d::Vector3D;
    ///
    /// let vector1 = Vector3D { x: 3.0, y: 2.0, z: 1.0 };
    /// let vector2 = Vector3D { x: 1.0, y: 4.0, z: 2.0 };
    /// let angle = vector1.angle_between(&vector2);
    /// ```
    pub fn angle_between(&self, other: &Vector3D) -> f64 {
        let dot_product = self.dot_product(other);
        let magnitude_product = self.magnitude() * other.magnitude();

        dot_product.acos() / magnitude_product
    }
}