# 为了学好Rust也是拼了系列-数学库-直线方程


直线是欧几里得几何学中的基本几何对象之一，可以用数学方式进行定义。

设有两个点 $$P_1(x_1, y_1)$$ 和 $$P_2(x_2, y_2)$$，这两点确定了一条直线。直线上的任意一点 $$P(x, y)$$ 都满足以下关系：

$$ \frac{y - y_1}{y_2 - y_1} = \frac{x - x_1}{x_2 - x_1} $$

上述等式称为点斜式，它表达了直线上的任意两点之间的比例关系。

另一种常见的定义是使用向量。设有一个点 $$P_1(x_1, y_1)$$ 和一个方向向量 $$\mathbf{v} = \langle a, b \rangle$$，那么直线上的任意一点 $$P(x, y)$$ 都满足以下关系：

$$ \langle x - x_1, y - y_1 \rangle = t \cdot \langle a, b \rangle $$

其中 $$t$$ 是任意实数。这个等式表示，直线上的任意一点到起始点 $$P_1$$ 的位移向量与方向向量成比例。

直线还可以通过一般式方程表示为 $$Ax + By + C = 0$$，其中 $$A$$、$$B$$、$$C$$ 是常数，且 $$A$$ 和 $$B$$ 不同时为零。这种表示方式更加抽象，适用于不同坐标系和维度中的直线。

常见表达方式

1. **点斜式：**

直线上的任意一点 $$ (x_1, y_1) $$ 和直线的斜率 $$ m $$ 决定了直线的方程。点斜式的方程为：

$$ y - y_1 = m(x - x_1) $$

其中 $$ (x_1, y_1) $$ 是直线上的一点，$$ m $$ 是直线的斜率。

2. **截距式：**

直线与坐标轴的交点称为截距，一般地，直线与 $$ y $$ 轴的交点为 $$ (0, b) $$，与 $$ x $$ 轴的交点为 $$ (a, 0) $$。直线的截距式方程为：

$$ y = mx + b $$

其中 $$ m $$ 是直线的斜率，$$ b $$ 是 $$ y $$ 轴的截距。

3. **一般式：**

直线的一般式方程为：

$$ Ax + By + C = 0 $$

其中 $$ A $$、$$ B $$ 和 $$ C $$ 是常数，且 $$ A $$ 和 $$ B $$ 不同时为零。这种形式的方程可以表示所有直线，包括垂直于 $$ y $$ 轴的直线。

这些是直线方程的一些基本形式。选择哪种形式通常取决于问题的要求和给定的信息。





程序使用一般式进行直线方程的表达，相关代码如下。

```
#[derive(Debug)]
pub struct LinearEquation {
    pub A: f64,
    pub B: f64,
    pub C: f64,
}
```







## 点斜式转一般式

点斜式到一般式的转换涉及到将点斜式的方程 $$y - y_1 = m(x - x_1)$$ 转换为一般式的方程 $$Ax + By + C = 0$$。以下是该转换的步骤：

假设点斜式的方程为 $$y - y_1 = m(x - x_1)$$，其中 $$(x_1, y_1)$$ 是直线上的一点，$$m$$ 是斜率。

1. 将点斜式方程展开得到标准形式：

   $$ y - y_1 = m(x - x_1) $$

   展开后得：

   $$ y - y_1 = mx - mx_1 $$

   将 $$y$$ 项移到右边：

   $$ y = mx - mx_1 + y_1 $$

2. 将方程整理为一般式 $$Ax + By + C = 0$$：

   $$ y = mx - mx_1 + y_1 $$

   移项得：

   $$ -mx + y = -mx_1 + y_1 $$

   令 $$A = -m, B = 1, C = -mx_1 + y_1$$，则一般式为：

   $$ -mx + y - (-mx_1 + y_1) = 0 $$

   简化得：

   $$ -mx + y + mx_1 - y_1 = 0 $$

   最终的一般式为：

   $$ mx_1 - y_1 - mx + y = 0 $$

   或者写成：

   $$ mx - y + (y_1 - mx_1) = 0 $$

因此，点斜式到一般式的转换结果为：

$$ mx - y + (y_1 - mx_1) = 0 $$



程序解如下

```
// 从点斜式参数创建一般式方程
// x1, y1 是直线上的一点
// slope 是直线的斜率
pub fn from_point_slope(x1: f64, y1: f64, slope: f64) -> Self {
    // 一般式方程的 A 系数为 -slope
    let A = -slope;
    // 一般式方程的 B 系数为 1
    let B = 1.0;
    // 一般式方程的 C 系数为 y1 - slope * x1
    let C = y1 - slope * x1;

    // 创建并返回一般式方程的实例
    LinearEquation { A, B, C }
}
```



## 一般式转点斜式

要将一般式 \(Ax + By + C = 0\) 的直线方程转换为点斜式 \(y = mx + b\) 形式，其中 \(m\) 是斜率，\((x_0, y_0)\) 是直线上的一点，可以按照以下步骤进行：

1. **解出 \(y\)：**
   将 \(Ax + By + C = 0\) 中的 \(By\) 移项，并除以 \(B\)，得到 \(y = -\frac{A}{B}x - \frac{C}{B}\)。

2. **标出斜率 \(m\) 和截距 \(b\)：**
   对比 \(y = -\frac{A}{B}x - \frac{C}{B}\) 与 \(y = mx + b\)，得到斜率 \(m = -\frac{A}{B}\)。

3. **找到直线上的一点 \((x_0, y_0)\)：**
   选择一个点，比如令 \(x_0 = 0\)，然后计算相应的 \(y_0\)，即 \(y_0 = -\frac{C}{B}\)。



**程序解如下**

```
pub fn to_point_slope_form(&self) -> Option<(f64, (f64, f64))> {
    if self.B != 0.0 {
        let slope = -self.A / self.B;
        let point = (0.0, -self.C / self.B);
        Some((slope, point))
    } else {
        None // 如果 B 为零，斜率不存在
    }
}
```







## 截距式转一般式

截距式方程表示为 $$y = mx + b$$，其中 $$m$$ 是斜率，$$b$$ 是 $$y$$ 轴的截距。将这个方程转换为一般式 $$Ax + By + C = 0$$ 的步骤如下：

1. 将截距式方程移项得到标准形式：

   $$ y - mx = b $$

2. 将 $$y$$ 项移到右边：

   $$ -mx + y = b $$

3. 令 $$A = -m, B = 1, C = -b$$，则一般式为：

   $$ -mx + y - b = 0 $$

4. 简化得：

   $$ -mx + y - b = 0 $$

因此，截距式到一般式的转换结果为：

$$ -mx + y - b = 0 $$



**程序解如下**

```
// 将截距式方程转换为一般式方程
pub fn from_slope_intercept(m: f64, b: f64) -> Self {
    // 一般式方程的 A 系数为 -m
    let A = -m;
    // 一般式方程的 B 系数为 1
    let B = 1.0;
    // 一般式方程的 C 系数为 -b
    let C = -b;

    // 创建并返回一般式方程的实例
    LinearEquation { A, B, C }
}
```



## 一般式转截距式

要将一般式 $$Ax + By + C = 0$$ 的直线方程转换为斜率截距形式（slope-intercept form）$$y = mx + b$$，其中 $$m$$ 是斜率，$$b$$ 是截距，可以按照以下步骤进行：

1. **解出 $$y$$：**
   将 $$Ax + By + C = 0$$ 中的 $$By$$ 移项，并除以 $$B$$，得到 $$y = -\frac{A}{B}x - \frac{C}{B}$$。

2. **标出斜率 $$m$$ 和截距 $$b$$：**
   对比 $$y = -\frac{A}{B}x - \frac{C}{B}$$ 与 $$y = mx + b$$，得到斜率 $$m = -\frac{A}{B}$$ 和截距 $$b = -\frac{C}{B}$$。

**程序解如下**

```
pub fn to_slope_intercept_form(&self) -> Option<(f64, f64)> {
    if self.B != 0.0 {
        let slope = -self.A / self.B;
        let intercept = -self.C / self.B;
        Some((slope, intercept))
    } else {
        None // 如果 B 为零，斜率不存在
    }
}
```

## 直线方程的平移

要进行直线方程的平移操作，可以通过修改方程中的常数项来实现。考虑一般式的直线方程 $$Ax + By + C = 0$$，现在要将直线沿着 x 轴和 y 轴方向分别平移 $$h$$ 和 $$k$$ 个单位。

1. **沿 x 轴平移：** 将 $$C$$ 增加 $$h$$，方程变为 $$Ax + By + (C + h) = 0$$。

2. **沿 y 轴平移：** 将 $$C$$ 增加 $$k$$，方程变为 $$Ax + By + (C + k) = 0$$。



**程序解如下**

```
// 将直线沿 x 轴平移 h 个单位，返回新的直线方程
pub fn translate_along_x(&self, h: f64) -> LinearEquation {
    LinearEquation {
        A: self.A,
        B: self.B,
        C: self.C + h,
    }
}

// 将直线沿 y 轴平移 k 个单位，返回新的直线方程
pub fn translate_along_y(&self, k: f64) -> LinearEquation {
    LinearEquation {
        A: self.A,
        B: self.B,
        C: self.C + k,
    }
}
```



## 直线方程的旋转（绕原点）

对于直线方程的旋转，可以使用旋转矩阵的方法。考虑一般式的直线方程 $$Ax + By + C = 0$$，以及旋转角度 $$\theta$$，新的直线方程可以通过以下步骤计算：

1. 定义旋转矩阵：

   $$ R = \begin{bmatrix} \cos(\theta) & -\sin(\theta) \\ \sin(\theta) & \cos(\theta) \end{bmatrix} $$

2. 定义向量 $$\mathbf{v} = \begin{bmatrix} x \\ y \end{bmatrix}$$ 表示原始直线上的点。

3. 计算旋转后的向量 $$\mathbf{v'}$$：

   $$ \mathbf{v'} = R \cdot \mathbf{v} $$

4. 将旋转后的向量代入一般式方程，得到新的直线方程。

**程序解如下**

```
// 将直线绕原点逆时针旋转 theta 弧度，返回新的直线方程
pub fn rotate_around_origin(&self, theta: f64) -> LinearEquation {
    let cos_theta = theta.cos();
    let sin_theta = theta.sin();

    // 定义旋转矩阵
    let rotation_matrix = [
        [cos_theta, -sin_theta],
        [sin_theta, cos_theta],
    ];

    // 计算新的系数
    let new_A = self.A * rotation_matrix[0][0] + self.B * rotation_matrix[0][1];
    let new_B = self.A * rotation_matrix[1][0] + self.B * rotation_matrix[1][1];
    let new_C = self.C;

    // 返回新的直线方程
    LinearEquation {
        A: new_A,
        B: new_B,
        C: new_C,
    }
}
```

## 直线方程的旋转（绕任意点）



要将直线绕任意一点旋转一定角度，可以采用平移和旋转的组合操作。以下是基于数学原理的步骤：

假设直线方程为 $$Ax + By + C = 0$$，要将其绕点 $$(x_0, y_0)$$ 逆时针旋转角度 $$\theta$$，可以按照以下步骤进行：

1. **平移操作：** 将直线方程平移到原点附近，即将点 $$(x_0, y_0)$$ 移动到原点。这可以通过将方程两边都减去 $$Ax_0 + By_0 + C$$ 来实现。

   $$ A(x - x_0) + B(y - y_0) = 0 $$

2. **旋转操作：** 对平移后的方程进行旋转，即使用旋转矩阵。对于逆时针旋转角度 $$\theta$$ 的旋转矩阵：

   $$ R = \begin{bmatrix} \cos(\theta) & -\sin(\theta) \\ \sin(\theta) & \cos(\theta) \end{bmatrix} $$

   旋转后的方程为：

   $$ (A \cos(\theta) - B \sin(\theta))(x - x_0) + (A \sin(\theta) + B \cos(\theta))(y - y_0) = 0 $$

3. **还原操作：** 将旋转后的方程还原到原始坐标系，即将点 $$(x_0, y_0)$$ 移回原来的位置。这可以通过在方程两边加上 $$Ax_0 + By_0 + C$$ 来实现。

   $$ (A \cos(\theta) - B \sin(\theta))(x - x_0) + (A \sin(\theta) + B \cos(\theta))(y - y_0) + Ax_0 + By_0 + C = 0 $$

**程序解如下**

```
// 将直线绕任意点逆时针旋转 theta 弧度，返回新的直线方程
pub fn rotate_around_point(&self, theta: f64, center: (f64, f64)) -> LinearEquation {
    // 计算旋转矩阵
    let cos_theta = theta.cos();
    let sin_theta = theta.sin();

    // 将直线平移到旋转中心
    let mut translated_line = self.translate(-center.0, -center.1);

    // 应用旋转矩阵
    let new_A = self.A * cos_theta - self.B * sin_theta;
    let new_B = self.A * sin_theta + self.B * cos_theta;

    // 更新新的系数
    translated_line.A = new_A;
    translated_line.B = new_B;

    // 将直线还原到原来的位置
    translated_line.translate(center.0, center.1)
}

// 将直线沿 x 轴平移 h 个单位，沿 y 轴平移 k 个单位
pub fn translate(&self, h: f64, k: f64) -> LinearEquation {
    LinearEquation {
        A: self.A,
        B: self.B,
        C: self.C + self.A * h + self.B * k,
    }
}
```





## X轴夹角Y轴夹角

已知直线的一般式方程为 $$Ax + By + C = 0$$，我们可以使用以下方法来求与 X 轴和 Y 轴的夹角：

1. **与 X 轴的夹角：** 直线与 X 轴的夹角可以通过斜率来求解。斜率 $$m$$ 可以通过一般式方程中的系数 $$A$$ 和 $$B$$ 计算：

   $$ m = -\frac{A}{B} $$

   直线与 X 轴的夹角 $$\alpha$$ 满足：

   $$ \tan(\alpha) = m $$

   因此，直线与 X 轴的夹角为 $$\alpha = \arctan(m)$$。

2. **与 Y 轴的夹角：** 直线与 Y 轴的夹角与与 X 轴的夹角之差为 90 度。因此，直线与 Y 轴的夹角为 $$90^\circ - \alpha$$。

**程序解如下**

```
// 计算直线与 X 轴和 Y 轴的夹角（弧度）
pub fn angles_with_axes(&self) -> (f64, f64) {
    // 计算斜率
    let slope = -self.A / self.B;

    // 计算与 X 轴的夹角
    let angle_with_x_axis = slope.atan();

    // 计算与 Y 轴的夹角
    let angle_with_y_axis = PI / 2.0 - angle_with_x_axis;

    (angle_with_x_axis, angle_with_y_axis)
}
```





## 直线和直线的关系

直线方程之间的关系涉及到它们的斜率和截距。以下是关于两条直线之间相交、平行和垂直关系的解释：

### 1. **相交关系：**
两条直线相交的条件是它们的斜率不相等，或者其中一条直线是垂直线。如果两条直线的斜率都存在且不相等，它们就相交。

数学表达：
$$ m_1 \neq m_2 \quad \text{或} \quad m_1 \cdot m_2 = -1 $$
其中，$$ m_1 $$ 和 $$ m_2 $$ 分别是两条直线的斜率。

### 2. **平行关系：**
两条直线平行的条件是它们的斜率相等，且不是垂直线。如果两条直线的斜率相等且不是垂直线，它们就平行。

数学表达：
$$ m_1 = m_2 \quad \text{且} \quad m_1 \cdot m_2 \neq -1 $$
其中，$$ m_1 $$ 和 $$ m_2 $$ 分别是两条直线的斜率。

### 3. **垂直关系：**
两条直线垂直的条件是它们的斜率乘积等于 -1。如果两条直线的斜率乘积等于 -1，它们就垂直。

数学表达：
$$ m_1 \cdot m_2 = -1 $$
其中，$$ m_1 $$ 和 $$ m_2 $$ 分别是两条直线的斜率。

### 4. **直线方程形式：**
两条直线的一般形式方程分别为：
$$ Ax + By + C_1 = 0 $$
$$ Ax + By + C_2 = 0 $$

### 5. **斜率和截距关系：**
两条直线的斜率分别为 $$ -\frac{A}{B} $$，它们平行的条件是这两个比例相等。两条直线的截距分别为 $$ -\frac{C_1}{B} $$ 和 $$ -\frac{C_2}{B} $$，它们平行的条件是这两个比例相等。



### 6. **垂直于 X 轴的直线：**
一条垂直于 X 轴的直线的斜率是不存在的（无穷大）。如果一条直线的斜率为无穷大，它与任何斜率存在的直线相交。两条垂直于 X 轴的直线的相交条件是它们的 X 截距相等。

数学表达：
$$ \text{如果直线 } A_1x + B_1y + C_1 = 0 \text{ 垂直于 X 轴，则 } B_1 = 0 $$

### 7. **垂直于 Y 轴的直线：**
一条垂直于 Y 轴的直线的斜率是零。两条垂直于 Y 轴的直线平行的条件是它们的 Y 截距相等。

数学表达：
$$ \text{如果直线 } A_1x + B_1y + C_1 = 0 \text{ 垂直于 Y 轴，则 } A_1 = 0 $$

### 8. **特殊情况下的相交和平行条件：**
   - 如果一条直线垂直于 X 轴，而另一条直线的斜率存在，则它们相交。
   - 如果一条直线垂直于 Y 轴，而另一条直线的斜率存在，则它们平行。



**程序解如下**

```
// 判断直线是否垂直于 X 轴
pub fn is_vertical_to_x_axis(&self) -> bool {
    self.B == 0.0
}

// 判断直线是否垂直于 Y 轴
pub fn is_vertical_to_y_axis(&self) -> bool {
    self.A == 0.0
}

// 判断两条直线是否相交
pub fn are_intersecting(&self, other: &LinearEquation) -> bool {
    !(self.is_parallel_to(other) || self.is_equal_to(other))
}

// 判断两条直线是否平行
pub fn are_parallel(&self, other: &LinearEquation) -> bool {
    self.A * other.B == self.B * other.A
}

// 判断两条直线是否垂直
pub fn are_perpendicular(&self, other: &LinearEquation) -> bool {
    self.A * other.A + self.B * other.B == 0.0
}

// 判断两条直线是否相等
pub fn is_equal_to(&self, other: &LinearEquation) -> bool {
    self.A == other.A && self.B == other.B && self.C == other.C
}

// 获取直线的斜率
pub fn slope(&self) -> Option<f64> {
    if self.is_vertical_to_x_axis() {
        None // 斜率不存在
    } else {
        Some(-self.A / self.B)
    }
}
```



## 点和直线的关系

判断点与直线的位置关系通常涉及将点的坐标代入直线方程，并观察方程的结果。点与直线的位置关系有三种可能的情况：

1. **点在直线上：** 如果点的坐标满足直线方程，即将点的坐标代入直线方程后等式成立，那么点在直线上。

   直线方程：$$Ax + By + C = 0$$，点 $$(x_0, y_0)$$ 在直线上的条件是 $$Ax_0 + By_0 + C = 0$$。

2. **点在直线的上方或下方：** 如果点的坐标代入直线方程后等式不成立，那么可以判断点在直线的上方还是下方。可以通过直线方程的符号来确定。

   - 如果 $$Ax_0 + By_0 + C > 0$$，则点在直线的上方。
   - 如果 $$Ax_0 + By_0 + C < 0$$，则点在直线的下方。

3. **点不在直线上：** 如果点的坐标代入直线方程后等式不成立，并且点不在直线的上方或下方，那么点不在直线上。



**程序解如下**

```
// 判断点与直线的位置关系
pub fn point_line_relationship(&self, point: &Point2D) -> PointLineRelationship {
    let result = self.A * point.x + self.B * point.y + self.C;

    if result == 0.0 {
        PointLineRelationship::OnLine
    } else if result > 0.0 {
        PointLineRelationship::AboveLine
    } else {
        PointLineRelationship::BelowLine
    }
}
```





## 直线和圆是否相切

要判断一条直线和一个圆是否相切，可以考虑直线与圆的位置关系。如果直线与圆相切，那么直线必定是圆的切线，且切点是唯一的。

给定一条直线的一般式方程 $$Ax + By + C = 0$$ 和一个圆的方程 $$(x - h)^2 + (y - k)^2 = r^2$$，其中 $$(h, k)$$ 是圆心的坐标，$$r$$ 是半径，可以通过以下步骤判断直线和圆是否相切：

1. 计算直线到圆心的距离：

   $$ d = \frac{|Ah + Bk + C|}{\sqrt{A^2 + B^2}} $$

2. 如果 $$d = r$$，则直线和圆相切。

**程序解如下**



```
// 判断直线与圆是否相切
pub fn is_tangent_to_circle(&self, circle: &Circle) -> bool {
    // 计算直线到圆心的距离
    let distance_to_center = (self.A * circle.x + self.B * circle.y + self.C).abs()
        / f64::sqrt(self.A.powi(2) + self.B.powi(2));

    // 判断是否相切（距离差小于 EPSILON，考虑浮点数误差）
    (distance_to_center - circle.radius).abs() < EPSILON
}
```
