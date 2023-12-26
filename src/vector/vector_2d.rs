// 定义二维向量结构体
pub struct Vector2D {
    pub x: f64,
    pub y: f64,
}

// 实现一个方法来创建新的二维向量
impl Vector2D {
    // 创建新的二维向量
    pub fn new(x: f64, y: f64) -> Vector2D {
        Vector2D { x, y }
    }

    // 计算二维向量的模（长度）
    pub fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }


    // 向量加法
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