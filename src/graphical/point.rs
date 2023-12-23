
#[derive(Debug, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point
{
    pub fn distance_between_points(&self, point2: Point) -> f64 {
        let dx = point2.x - self.x;
        let dy = point2.y - self.y;
        (dx * dx + dy * dy).sqrt()
    }

    /// 旋转
    pub fn rotate(&self, angle: f64) -> Point {
        let cos_theta = angle.cos();
        let sin_theta = angle.sin();

        let new_x = self.x * cos_theta - self.y * sin_theta;
        let new_y = self.x * sin_theta + self.y * cos_theta;

        Point { x: new_x, y: new_y }
    }
}
