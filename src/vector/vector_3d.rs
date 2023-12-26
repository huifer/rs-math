#[derive(Debug)]
pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3D {
    // 向量加法
    pub fn add(&self, other: &Vector3D) -> Vector3D {
        Vector3D {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    // 向量减法
    pub fn subtract(&self, other: &Vector3D) -> Vector3D {
        Vector3D {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
    // 数量乘法
    pub fn scalar_multiply(&self, scalar: f64) -> Vector3D {
        Vector3D {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
    // 计算叉积
    pub fn cross_product(&self, other: &Vector3D) -> Vector3D {
        Vector3D {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    // 计算两个向量的点积
    pub fn dot_product(&self, other: &Vector3D) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    // 计算模长
    pub fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }
    // 计算两个向量之间的夹角（弧度）
    pub fn angle_between(&self, other: &Vector3D) -> f64 {
        let dot_product = self.dot_product(other);
        let magnitude_product = self.magnitude() * other.magnitude();

        dot_product.acos() / magnitude_product
    }
}