// 直线方程
#[derive(Debug)]
pub struct LinearEquation {
    pub A: f64,
    pub B: f64,
    pub C: f64,
}

impl LinearEquation {
    // 通过两点计算切线方程的一般式表示
    fn from_points(x1: f64, y1: f64, x2: f64, y2: f64) -> LinearEquation {
        let A = y2 - y1;
        let B = x1 - x2;
        let C = x2 * y1 - x1 * y2;

        LinearEquation {
            A,
            B,
            C,
        }
    }


    /// 计算圆弧切线方程的一般式表示
    pub fn from_arc(radius: f64, x1: f64, y1: f64, x2: f64, y2: f64) -> LinearEquation {
        // 计算圆心坐标
        let center_x = (x1 + x2) / 2.0;
        let center_y = (y1 + y2) / 2.0;

        // 计算切线斜率
        let m = (y2 - y1) / (x2 - x1);

        // 计算切线方程的斜率和截距
        let a = -m;
        let b = 1.0;
        let c = -a * center_x - b * center_y;

        LinearEquation { A: a, B: b, C: c }
    }
}