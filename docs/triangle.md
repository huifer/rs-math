# 为了学好Rust也是拼了系列-数学库-三角形


## 定义

三角形是一个具有三条边和三个角的几何形状。以下是三角形的一些基本特性：

1. **三边：** 三角形有三条边，分别记为$$a$$、$$b$$和$$c$$。

2. **三角形的内角和：** 任何三角形的内角和都等于180度（或π弧度）。这可以表示为等式：
   $$ \angle A + \angle B + \angle C = 180^\circ \text{ 或 } \pi \text{ 弧度} $$

3. **三角形的分类：** 三角形根据边和角的性质可以分为不同的类型，包括：
    - **等边三角形：** 三条边都相等。
    - **等腰三角形：** 至少两条边相等。
    - **直角三角形：** 有一个90度的角。
    - **锐角三角形：** 三个角都小于90度。
    - **钝角三角形：** 有一个角大于90度。

4. **勾股定理：** 对于直角三角形，勾股定理成立，即：
   $$ a^2 + b^2 = c^2 $$
   这里，$$c$$是斜边，$$a$$和$$b$$是直角边。

5. **三角形的面积：** 三角形的面积可以用以下公式之一来计算：
    - 使用三角形的底和高：$$ \text{面积} = \frac{1}{2} \times \text{底} \times \text{高} $$
    - 使用三边的长度（海伦公式）：$$ \text{面积} = \sqrt{s \cdot (s-a) \cdot (s-b) \cdot (s-c)} $$
      这里，$$s$$是半周长，$$s = \frac{a+b+c}{2}$$。



Rust结构如下

```
#[derive(Debug, PartialEq)]
pub struct Triangle {
    pub vertex_a: Point,
    pub vertex_b: Point,
    pub vertex_c: Point,
}
```



## 内切圆

内切圆是一个与三角形的三条边都相切的圆。设三角形的三边长度分别为 $$a$$、$$b$$ 和 $$c$$，对应的三个内角为 $$\angle A$$、$$\angle B$$ 和 $$\angle C$$。

内切圆半径 $$r$$ 可以通过以下方式计算：

1. **半周长 $$s$$:** 首先计算三角形的半周长，即 $$s = \frac{a + b + c}{2}$$。

2. **内切圆半径 $$r$$:** 利用半周长，内切圆半径可以通过以下公式计算：
   $$ r = \frac{\text{三角形的面积}}{s} $$

3. **三角形的面积 $$A$$:** 三角形的面积可以使用海伦公式计算：
   $$ A = \sqrt{s \cdot (s - a) \cdot (s - b) \cdot (s - c)} $$

**程序解如下**

```
impl Triangle {
    // 构造函数，创建一个新的三角形实例
    pub fn new(a: Point, b: Point, c: Point) -> Self {
        Triangle {
            vertex_a: a,
            vertex_b: b,
            vertex_c: c,
        }
    }

    // 计算三角形的边长
    pub fn side_length(&self, start: &Point, end: &Point) -> f64 {
        ((end.x - start.x).powi(2) + (end.y - start.y).powi(2)).sqrt()
    }

    // 计算三角形的周长
    pub fn perimeter(&self) -> f64 {
        let side_ab = self.side_length(&self.vertex_a, &self.vertex_b);
        let side_bc = self.side_length(&self.vertex_b, &self.vertex_c);
        let side_ca = self.side_length(&self.vertex_c, &self.vertex_a);
        side_ab + side_bc + side_ca
    }

    // 计算三角形的面积
    pub fn area(&self) -> f64 {
        let side_ab = self.side_length(&self.vertex_a, &self.vertex_b);
        let side_bc = self.side_length(&self.vertex_b, &self.vertex_c);
        let side_ca = self.side_length(&self.vertex_c, &self.vertex_a);
        let s = self.perimeter() / 2.0; // 半周长
        (s * (s - side_ab) * (s - side_bc) * (s - side_ca)).sqrt()
    }


    // 计算三角形的半周长
    fn semi_perimeter(&self) -> f64 {
        let side_ab = self.side_length(&self.vertex_a, &self.vertex_b);
        let side_bc = self.side_length(&self.vertex_b, &self.vertex_c);
        let side_ca = self.side_length(&self.vertex_c, &self.vertex_a);
        (side_ab + side_bc + side_ca) / 2.0
    }


    // 计算内切圆
    pub fn in_circle(&self) -> Circle {
        let s = self.semi_perimeter();
        let side_ab = self.side_length(&self.vertex_a, &self.vertex_b);
        let side_bc = self.side_length(&self.vertex_b, &self.vertex_c);
        let side_ca = self.side_length(&self.vertex_c, &self.vertex_a);

        let radius = self.area() / s;

        // 计算内切圆的中心坐标
        let center_x = (side_bc * self.vertex_a.x + side_ca * self.vertex_b.x + side_ab * self.vertex_c.x) / (side_ab + side_bc + side_ca);
        let center_y = (side_bc * self.vertex_a.y + side_ca * self.vertex_b.y + side_ab * self.vertex_c.y) / (side_ab + side_bc + side_ca);

        Circle {
            x: center_x,
            y: center_y,
            radius,
        }
    }
}
```



## 旋转三角形

当一个点 $$(x, y)$$ 绕着另一个点 $$(h, k)$$ 逆时针旋转一个角度 $$\theta$$ 时，新的坐标 $$(x', y')$$ 可以通过以下公式计算得到：

$$
\begin{align*}
x' &= (x - h) \cos(\theta) - (y - k) \sin(\theta) + h \\
y' &= (x - h) \sin(\theta) + (y - k) \cos(\theta) + k
\end{align*}
$$

在这里，$$\cos(\theta)$$ 和 $$\sin(\theta)$$ 分别是旋转角度 $$\theta$$ 的余弦和正弦值。

对于三角形，每个顶点 $$(x, y)$$ 绕着中心点 $$(h, k)$$ 逆时针旋转一个角度 $$\theta$$ 后的新坐标可以通过上述公式计算得到。

```
// 绕着指定点旋转三角形，返回新的三角形实例
pub fn rotate_around_point(&self, center: &Point, angle_degrees: f64) -> Triangle {
    // 将角度转换为弧度
    let angle_radians = angle_degrees.to_radians();

    // 定义旋转矩阵
    let rotation_matrix = [
        [angle_radians.cos(), -angle_radians.sin()],
        [angle_radians.sin(), angle_radians.cos()],
    ];

    // 对每个顶点进行旋转
    let new_vertex_a = self.rotate_point(&self.vertex_a, center, rotation_matrix);
    let new_vertex_b = self.rotate_point(&self.vertex_b, center, rotation_matrix);
    let new_vertex_c = self.rotate_point(&self.vertex_c, center, rotation_matrix);

    Triangle {
        vertex_a: new_vertex_a,
        vertex_b: new_vertex_b,
        vertex_c: new_vertex_c,
    }
}
/// 旋转单个点
fn rotate_point(&self, point: &Point, center: &Point, matrix: [[f64; 2]; 2]) -> Point {
    let translated_x = point.x - center.x;
    let translated_y = point.y - center.y;

    let rotated_x = matrix[0][0] * translated_x + matrix[0][1] * translated_y;
    let rotated_y = matrix[1][0] * translated_x + matrix[1][1] * translated_y;

    Point {
        x: rotated_x + center.x,
        y: rotated_y + center.y,
    }
}
```



## 判断点是否在三角形内

判断一个点是否在三角形内可以使用重心坐标法。重心坐标是一组数值，描述了一个点在三角形内的位置。在一个平面三角形 ABC 中，给定点 P 的重心坐标 $$ (p_1, p_2, p_3) $$ 满足以下条件：

1. $$ p_1 + p_2 + p_3 = 1 $$
2. $$ p_1 \cdot A + p_2 \cdot B + p_3 \cdot C = P $$

其中，A、B、C 是三角形的三个顶点，P 是要判断的点。

如果 $$ 0 \leq p_1, p_2, p_3 \leq 1 $$，则点 P 在三角形内。如果有一个 $$ p_i < 0 $$ 或 $$ p_i > 1 $$，则点 P 在三角形外。

```
// 判断点是否在三角形内
pub fn point_inside_triangle(&self, p: &Point) -> bool {
    // 计算重心坐标
    let barycentric_coords = self.barycentric_coordinates(p);

    // 判断点是否在三角形内
    barycentric_coords.iter().all(|&coord| coord >= 0.0 && coord <= 1.0)
}

// 辅助方法，计算点的重心坐标
pub fn barycentric_coordinates(&self, p: &Point) -> [f64; 3] {
    // 计算三个子三角形的面积
    let area_triangle = self.area();
    let area_sub_triangle_a = Triangle::new(p.clone(), self.vertex_b.clone(), self.vertex_c.clone()).area();
    let area_sub_triangle_b = Triangle::new(self.vertex_a.clone(), p.clone(), self.vertex_c.clone()).area();
    let area_sub_triangle_c = Triangle::new(self.vertex_a.clone(), self.vertex_b.clone(), p.clone()).area();

    // 计算重心坐标
    let p1 = area_sub_triangle_a / area_triangle;
    let p2 = area_sub_triangle_b / area_triangle;
    let p3 = area_sub_triangle_c / area_triangle;

    [p1, p2, p3]
}
```





## 垂心、重心、内心、外心计算

垂心、重心、内心、外心的数学解法，我们将考虑一个一般的三角形，其中顶点分别为 $$A(x_1, y_1)$$, $$B(x_2, y_2)$$, $$C(x_3, y_3)$$。

1. **垂心（Orthocenter）：**
   - 垂心是三角形的三条高的交点。
   - 设 $$H$$ 为垂心的坐标，分别在三条高 $$BC: x = x_1$$, $$AC: y = y_2$$, $$AB: y = y_3 - \frac{(x_3 - x_1)(y_1 - y_2)}{x_2 - x_1}$$ 上求交点。
   - 解方程组：
     $$ x = x_1 $$
     $$ y = y_2 $$
     $$ y = y_3 - \frac{(x_3 - x_1)(y_1 - y_2)}{x_2 - x_1} $$
   - 求解得到垂心坐标 $$H(x_H, y_H)$$。

2. **重心（Centroid）：**
   - 重心是三角形的三条中线的交点，中线是每个顶点和对边中点的连线。
   - 重心坐标为 $$G\left(\frac{x_1 + x_2 + x_3}{3}, \frac{y_1 + y_2 + y_3}{3}\right)$$。

3. **内心（Incenter）：**
   - 内心是三角形的三条角平分线的交点。
   - 设 $$I$$ 为内心的坐标，角平分线的方程可以通过点斜式 $$y - y_1 = m(x - x_1)$$ 求解，其中 $$m$$ 是角平分线的斜率。
   - 设 $$a$$, $$b$$, $$c$$ 为三边的长度，$$s$$ 为半周长。则角平分线的斜率 $$m$$ 分别为：
     $$ m_A = \frac{y_2 - y_1}{x_2 - x_1} $$
     $$ m_B = \frac{y_3 - y_2}{x_3 - x_2} $$
     $$ m_C = \frac{y_1 - y_3}{x_1 - x_3} $$
   - 解方程组：
     $$ y - y_1 = m_A \cdot (x - x_1) $$
     $$ y - y_2 = m_B \cdot (x - x_2) $$
     $$ y - y_3 = m_C \cdot (x - x_3) $$
   - 求解得到内心坐标 $$I(x_I, y_I)$$。

4. **外心（Circumcenter）：**
   - 外心是三角形的三条边的垂直平分线的交点。
   - 设 $$O$$ 为外心的坐标，垂直平分线的方程可以通过求两个边中点的斜率和斜率的负倒数得到。
   - 设 $$M_{AB}$$, $$M_{BC}$$, $$M_{CA}$$ 分别为 $$AB$$, $$BC$$, $$CA$$ 中点。垂直平分线的斜率分别为：
     $$ m_{AB} = -\frac{x_2 - x_1}{y_2 - y_1} $$
     $$ m_{BC} = -\frac{x_3 - x_2}{y_3 - y_2} $$
     $$ m_{CA} = -\frac{x_1 - x_3}{y_1 - y_3} $$
   - 解方程组：
     $$ y - \frac{y_1 + y_2}{2} = m_{AB} \cdot (x - \frac{x_1 + x_2}{2}) $$
     $$ y - \frac{y_2 + y_3}{2} = m_{BC} \cdot (x - \frac{x_2 + x_3}{2}) $$
     $$ y - \frac{y_3 + y_1}{2} = m_{CA} \cdot (x - \frac{x_3 + x_1}{2}) $$
   - 求解得到外心坐标 $$O(x_O, y_O)$$。



**程序解如下**



```
// 计算垂心
pub fn orthocenter(&self) -> Point {
    let x_h = self.vertex_a.x;
    let y_h = self.vertex_b.y;
    Point { x: x_h, y: y_h }
}

// 计算重心
pub fn centroid(&self) -> Point {
    let x_g = (self.vertex_a.x + self.vertex_b.x + self.vertex_c.x) / 3.0;
    let y_g = (self.vertex_a.y + self.vertex_b.y + self.vertex_c.y) / 3.0;
    Point { x: x_g, y: y_g }
}

// 计算内心
pub fn incenter(&self) -> Point {
    let a = self.vertex_b.distance_to(&self.vertex_c);
    let b = self.vertex_c.distance_to(&self.vertex_a);
    let c = self.vertex_a.distance_to(&self.vertex_b);


    let x_i = (a * self.vertex_a.x + b * self.vertex_b.x + c * self.vertex_c.x) / (a + b + c);
    let y_i = (a * self.vertex_a.y + b * self.vertex_b.y + c * self.vertex_c.y) / (a + b + c);

    Point { x: x_i, y: y_i }
}

// 计算外心
pub fn circumcenter(&self) -> Point {
    let m_ab = Point {
        x: (self.vertex_a.x + self.vertex_b.x) / 2.0,
        y: (self.vertex_a.y + self.vertex_b.y) / 2.0,
    };

    let m_bc = Point {
        x: (self.vertex_b.x + self.vertex_c.x) / 2.0,
        y: (self.vertex_b.y + self.vertex_c.y) / 2.0,
    };

    let m_ca = Point {
        x: (self.vertex_c.x + self.vertex_a.x) / 2.0,
        y: (self.vertex_c.y + self.vertex_a.y) / 2.0,
    };

    let m_ab_slope = -(self.vertex_b.x - self.vertex_a.x) / (self.vertex_b.y - self.vertex_a.y);
    let m_bc_slope = -(self.vertex_c.x - self.vertex_b.x) / (self.vertex_c.y - self.vertex_b.y);
    let m_ca_slope = -(self.vertex_a.x - self.vertex_c.x) / (self.vertex_a.y - self.vertex_c.y);

    let x_o = (m_ab.y - m_ab_slope * m_ab.x + m_bc.y - m_bc_slope * m_bc.x + m_ca.y - m_ca_slope * m_ca.x)
        / (m_ab_slope + m_bc_slope + m_ca_slope);

    let y_o = m_ab_slope * (x_o - m_ab.x) + m_ab.y;

    Point { x: x_o, y: y_o }
}
```

## 判断三角形种类

已知三个点求这个三角形属于直角、锐角、钝角中的哪一种，计算逻辑如下。

1. **计算两点之间的距离：**
   - 两点 $$P_1(x_1, y_1)$$ 和 $$P_2(x_2, y_2)$$ 之间的距离可以使用欧几里得距离公式计算：
     $$ d = \sqrt{(x_2 - x_1)^2 + (y_2 - y_1)^2} $$

2. **计算三角形的角度：**
   - 对于三角形ABC，以A为例，可以使用余弦定理计算角A（$$ \angle A $$）：
     $$ \cos(\angle A) = \frac{b^2 + c^2 - a^2}{2bc} $$
     其中，$$ a $$, $$ b $$, $$ c $$ 分别是BC、CA、AB三边的长度。

3. **弧度与角度的转换：**
   - 使用反余弦函数将余弦值转换为角度：
     $$ \text{angle\_rad} = \cos^{-1}(\cos(\angle A)) $$
     $$ \text{angle\_deg} = \text{angle\_rad} \times \frac{180}{\pi} $$

4. **分类三角形：**
   - 如果三个角都小于90度，那么这是一个锐角三角形。
   - 如果有一个角等于90度，那么这是一个直角三角形。
   - 如果有一个角大于90度，那么这是一个钝角三角形。

**程序解如下**



```
// 计算三角形的角度
pub fn classify_triangle(&self) -> TriangleType {
    let (angle_a, angle_b, angle_c) = self.classify_angles();

    if angle_a < 90.0 && angle_b < 90.0 && angle_c < 90.0 {
        TriangleType::Acute
    } else if angle_a == 90.0 || angle_b == 90.0 || angle_c == 90.0 {
        TriangleType::Right
    } else {
        TriangleType::Obtuse
    }
}
// 计算三角形的角度
pub fn classify_angles(&self) -> (f64, f64, f64) {
    let angle_a = self.angle_between(&self.vertex_b, &self.vertex_c, &self.vertex_a);
    let angle_b = self.angle_between(&self.vertex_a, &self.vertex_c, &self.vertex_b);
    let angle_c = self.angle_between(&self.vertex_a, &self.vertex_b, &self.vertex_c);

    (angle_a, angle_b, angle_c)
}

// 计算两条边之间的角度
fn angle_between(&self, p1: &Point, p2: &Point, p3: &Point) -> f64 {
    let a = p1.distance_to(p2);
    let b = p2.distance_to(p3);
    let c = p3.distance_to(p1);

    let cos_angle = (a.powi(2) + b.powi(2) - c.powi(2)) / (2.0 * a * b);
    let angle_rad = cos_angle.acos();

    // 将弧度转换为角度
    let angle_deg = angle_rad.to_degrees();

    angle_deg
}
```
