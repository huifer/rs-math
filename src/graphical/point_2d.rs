#[derive(Debug, PartialEq, Clone)]
pub struct Point2D {
    pub x: f64,
    pub y: f64,
}

#[allow(dead_code)]
impl Point2D
{
    pub fn distance_between_points(&self, point2: Point2D) -> f64 {
        let dx = point2.x - self.x;
        let dy = point2.y - self.y;
        (dx * dx + dy * dy).sqrt()
    }

    /// 旋转
    pub fn rotate(&self, angle: f64) -> Point2D {
        let cos_theta = angle.cos();
        let sin_theta = angle.sin();

        let new_x = self.x * cos_theta - self.y * sin_theta;
        let new_y = self.x * sin_theta + self.y * cos_theta;

        Point2D { x: new_x, y: new_y }
    }

    pub fn distance_to(&self, other: &Point2D) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }

    // 两点相加
    pub fn add_points(p1: &Point2D, p2: &Point2D) -> Point2D {
        Point2D { x: p1.x + p2.x, y: p1.y + p2.y }
    }
}
