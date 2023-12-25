# 为了学好Rust也是拼了系列-数学库-圆


在几何学的世界中，圆是一种异常引人注目的完美形状。其优雅的外观和独特的数学属性使其成为自然界、艺术和科学领域中无法忽视的存在。圆的定义简单而直观，它是平面上所有到一个固定点（圆心）距离相等的点的集合。然而，这个简单的概念却涵盖了丰富的数学理论和实际应用。本文将带领读者深入探索圆的奥秘，从基本的几何特性到实际应用的领域，揭示这个几何学之美的独特魅力。随着我们逐步揭开圆的层层面纱，让我们一同沉浸在这个数学之美的宇宙中。



## 圆的定义
圆的定义简单而直观，它是平面上所有到一个固定点（圆心）距离相等的点的集合。

我们可以将圆的定义转化为数学公式，即：

$$
(x - a)^2 + (y - b)^2 = r^2
$$

其中，$a$ 和 $b$ 是圆心的坐标，$r$ 是半径。


用 Rust 定义圆可以使用如下程序

```rust
#[derive(Debug, PartialEq)]
pub struct Circle {
    /// 圆的中心坐标
    pub x: f64,
    pub y: f64,
    /// 圆的半径
    pub radius: f64,
}

```

## 求圆

在知道元的定义后通常情况下需要对圆进行求解。

### 两个点求圆

已知两个点和半径，可以通过以下步骤求解圆的方程：

1. **计算圆心坐标：**
   - 设已知的两个点分别为 $$ (x_1, y_1) $$ 和 $$ (x_2, y_2) $$，半径为 $$ r $$。
   - 圆心的坐标可以通过取两个点的中点来得到：
      $$ \text{圆心} = \left( \frac{x_1 + x_2}{2}, \frac{y_1 + y_2}{2} \right) $$

2. **计算圆的方程：**
   - 圆的标准方程为：
      $$ (x - h)^2 + (y - k)^2 = r^2 $$
      其中 $$ (h, k) $$ 是圆心的坐标。
   - 将圆心坐标代入方程，得到具体的圆方程。

综合以上步骤，整个过程可以用以下公式表示：

$$ (x - \frac{x_1 + x_2}{2})^2 + (y - \frac{y_1 + y_2}{2})^2 = r^2 $$

这个方程描述的就是以已知两点为直径的圆。请确保 $$ r $$ 是两点之间的距离的一半。



**程序解如下**

```
/// 通过两点和半径创建圆
pub fn from_points_and_radius(point1: &Point, point2: &Point, radius: f64) -> Option<Circle> {
    // 计算圆心的中点坐标
    let center_x = (point1.x + point2.x) / 2.0;
    let center_y = (point1.y + point2.y) / 2.0;

    // 计算两点间的距离
    let distance = ((point2.x - point1.x).powi(2) + (point2.y - point1.y).powi(2)).sqrt();

    info!("圆的半径为 {}，圆心为 ({}, {})", radius, center_x, center_y);
    // 验证半径是否有效
    if radius == distance / 2.0 {
        Some(Circle {
            x: center_x,
            y: center_y,
            radius,
        }
        )
    } else { None }
}
```

### 三点求圆

已知三个点，可以通过以下步骤求解包含这三个点的圆的方程：

1. **计算两条中垂线的交点：**
   - 选择任意两对点，计算它们的中点，然后计算它们的斜率的负倒数，即中垂线的斜率。
   - 通过中点和斜率可以得到中垂线的方程。对于另外两对点也做同样的计算。
   - 解这两条中垂线的方程，得到它们的交点，这个交点就是所求圆的圆心。

2. **计算圆的半径：**
   - 使用圆心和任意一个已知点的距离作为半径。距离的计算可以使用两点之间的距离公式：
      $$ r = \sqrt{(x_1 - h)^2 + (y_1 - k)^2} $$
      其中 $$ (h, k) $$ 是圆心的坐标，$$ (x_1, y_1) $$ 是已知点的坐标。

3. **写出圆的方程：**
   - 圆的标准方程为：
      $$ (x - h)^2 + (y - k)^2 = r^2 $$
      将圆心坐标 $$ (h, k) $$ 和半径 $$ r $$ 代入方程即可。

程序解如下

```
/// 三个点算圆
pub fn from_points(p1: &Point, p2: &Point, p3: &Point) -> Option<Circle> {
    // 计算圆心坐标 (h, k)
    let h = (p1.x + p2.x) / 2.0;
    let k = (p1.y + p2.y) / 2.0;

    // 计算半径 r
    let r = ((p1.x - h).powi(2) + (p1.y - k).powi(2)).sqrt();

    // 检查第三个点是否在圆上
    let distance_to_center_squared = (p3.x - h).powi(2) + (p3.y - k).powi(2);
    let epsilon = 1e-6; // 设置一个小的误差范围

    if (distance_to_center_squared - r.powi(2)).abs() < epsilon {
        Some(Circle { x: h, y: k, radius: r })
    } else {
        None
    }
}
```



## 圆上的点

通过计算圆上的点可以在一个二维平面内将一个圆的轮廓进行绘制。

计算圆上的点可以使用圆的参数方程，参数方程描述了圆上每个点的坐标。圆的参数方程为：

$$ x = h + r \cdot \cos(\theta) $$
$$ y = k + r \cdot \sin(\theta) $$

其中：
- $$(h, k)$$ 是圆心的坐标，
- $$r$$ 是圆的半径，
- $$\theta$$ 是极角（polar angle），在 $$0$$ 到 $$2\pi$$ 的范围内变化。

通过选择不同的 $$\theta$$ 值，就可以得到圆上的不同点的坐标。这是因为 $$\cos(\theta)$$ 和 $$\sin(\theta)$$ 分别给出了在极坐标系中的点与原点之间的水平和垂直距离。

举例说明，假设圆的圆心是 $$(0, 0)$$，半径是 $$1$$，我们可以通过以下方式计算圆上的点：

1. 当 $$\theta = 0$$ 时，$$x = 0 + 1 \cdot \cos(0) = 1$$，$$y = 0 + 1 \cdot \sin(0) = 0$$。因此，$$(1, 0)$$ 是圆上的一个点。
2. 当 $$\theta = \frac{\pi}{2}$$ 时，$$x = 0 + 1 \cdot \cos(\frac{\pi}{2}) = 0$$，$$y = 0 + 1 \cdot \sin(\frac{\pi}{2}) = 1$$。因此，$$(0, 1)$$ 是圆上的一个点。
3. 当 $$\theta = \pi$$ 时，$$x = 0 + 1 \cdot \cos(\pi) = -1$$，$$y = 0 + 1 \cdot \sin(\pi) = 0$$。因此，$$(-1, 0)$$ 是圆上的一个点。

通过不断改变 $$\theta$$ 值，可以得到圆上的其他点。在实际编程中，可以通过循环生成一系列 $$\theta$$ 值，然后根据参数方程计算相应的 $$x$$ 和 $$y$$ 坐标，从而得到圆上的点。



**程序解如下**

```
pub fn generate_points_on_circle(center_x: f64, center_y: f64, radius: f64, num_points: usize) -> Vec<Point> {
    // 存储生成的点的容器
    let mut points = Vec::with_capacity(num_points);

    // 计算角度步长，确保点在圆上均匀分布
    let angle_step = 2.0 * PI / num_points as f64;

    // 生成点的坐标
    for i in 0..num_points {
        let angle = i as f64 * angle_step;
        let x = center_x + radius * angle.cos();
        let y = center_y + radius * angle.sin();
        points.push(Point { x, y });
    }

    points
}
```



## 圆的关系



### 圆与圆相交

要判断两个圆是否相交，可以利用它们之间的距离和半径之间的关系。设两个圆的圆心分别为 $$(x_1, y_1)$$ 和 $$(x_2, y_2)$$，半径分别为 $$r_1$$ 和 $$r_2$$。

两个圆相交的条件是它们之间的距离小于两个圆的半径之和，即：

$$ \sqrt{(x_2 - x_1)^2 + (y_2 - y_1)^2} \leq r_1 + r_2 $$

如果上述不等式成立，两个圆相交；否则，它们不相交。

具体的步骤如下：

1. 计算两个圆心之间的距离 $$d$$：

$$ d = \sqrt{(x_2 - x_1)^2 + (y_2 - y_1)^2} $$

2. 判断是否满足相交条件：

$$ d \leq r_1 + r_2 $$

如果不等式成立，说明两个圆相交；如果不成立，说明两个圆不相交。

**程序解如下**

```
pub fn circles_intersect(&self, other: &Circle) -> bool {
    let distance_between_centers = ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt();
    let sum_of_radii = self.radius + other.radius;

    distance_between_centers < sum_of_radii
}
```

### 圆与圆相切

对于两个圆是否相切，数学上的条件是它们之间的距离等于两个圆的半径之和。形式化表示为：

$$ \sqrt{(x_2 - x_1)^2 + (y_2 - y_1)^2} = r_1 + r_2 $$

其中，$$(x_1, y_1)$$ 和 $$(x_2, y_2)$$ 分别是两个圆的圆心坐标，$$r_1$$ 和 $$r_2$$ 是它们的半径。

如果上述等式成立，那么两个圆相切；如果不成立，它们不相切。这是数学上判断两个圆是否相切的基本条件。

**程序解如下**

```
/// 判断两个圆是否相切
/// 它们的圆心之间的距离等于两个圆半径之和
/// ∣distance(center1,center2)−(radius1+radius2)∣<ϵ
pub fn circles_touch(&self, other: &Circle) -> bool {
    let distance_between_centers = ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt();
    let sum_of_radii = self.radius + other.radius;

    (distance_between_centers - sum_of_radii).abs() < f64::EPSILON
}
```



### 是否包含一个圆

要判断一个圆是否包含另一个圆，可以检查两个圆心之间的距离和半径的关系。设两个圆的圆心分别为 $$(x_1, y_1)$$ 和 $$(x_2, y_2)$$，半径分别为 $$r_1$$ 和 $$r_2$$。圆1包含圆2的条件是：

$$ \sqrt{(x_2 - x_1)^2 + (y_2 - y_1)^2} + r_2 \leq r_1 $$

如果上述不等式成立，说明圆2完全包含在圆1内部；否则，圆2不被圆1包含。

**程序解如下**

```rust
/// 判断一个圆是否完全包含另一个圆
/// 它们的圆心之间的距离加上小圆半径小于等于大圆半径
/// distance(center1,center2)+radius2≤radius1
pub fn circle_contains(&self, other: &Circle) -> bool {
    let distance_between_centers = ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt();
    distance_between_centers + other.radius <= self.radius
}

```

