# 为了学好Rust也是拼了系列-数学库-椭圆

椭圆是平面上的一种几何图形，其定义基于到两个固定点（焦点）的距离之和与到一个常数（长轴长度）的比较。让我们用数学方式来介绍椭圆。

椭圆的标准方程为：

$$ \frac{(x - h)^2}{a^2} + \frac{(y - k)^2}{b^2} = 1 $$

其中 $$(h, k)$$ 是椭圆的中心，$$a$$ 和 $$b$$ 分别是椭圆的长轴和短轴的长度。椭圆上的点 $$(x, y)$$ 满足上述方程。

在这个方程中，如果 $$a = b$$，椭圆就变成了圆。而当 $$a$$ 大于 $$b$$ 时，椭圆的形状更加狭长，长轴在 x 轴上。相反，当 $$b$$ 大于 $$a$$ 时，椭圆更加扁平，长轴在 y 轴上。

焦点与长轴之间的关系是通过椭圆的离心率来描述的，离心率 $$e$$ 的计算公式为：

$$ e = \sqrt{1 - \frac{b^2}{a^2}} $$

离心率是一个衡量椭圆形状的参数，取值范围为 $$0 \leq e < 1$$。当离心率为零时，椭圆是一个圆。

**程序定义如下**

```
#[derive(Debug, PartialEq,Clone,Copy)]
struct Ellipse {
    h: f64, // 椭圆中心的 x 坐标
    k: f64, // 椭圆中心的 y 坐标
    a: f64, // 长轴的长度
    b: f64, // 短轴的长度
}
```





## 面积

椭圆的面积计算使用的是椭圆的标准面积公式。椭圆的面积 $$A$$ 可以由椭圆的半长轴 $$a$$ 和半短轴 $$b$$ 长度计算得到。椭圆的面积公式如下：

$$ A = \pi \cdot a \cdot b $$

其中，$$\pi$$ 是圆周率，约等于 3.14159。 $$a$$ 是椭圆的半长轴的长度，$$b$$ 是椭圆的半短轴的长度。

**程序解如下**

```
// 计算椭圆的面积
pub fn area(&self) -> f64 {
    PI * self.a * self.b
}
```





## 周长

计算椭圆周长的准确公式较为复杂，通常需要通过数值方法或级数展开进行估算。然而，可以使用椭圆的周长估算公式，该公式提供了一个近似值。

椭圆的周长估算公式为：

$$ C \approx \pi \left[ 3(a+b) - \sqrt{(3a + b)(a + 3b)} \right] $$

其中，$$a$$ 和 $$b$$ 分别是椭圆的半长轴和半短轴的长度。

**程序解如下**

```
// 估算椭圆的周长
pub fn estimate_circumference(&self) -> f64 {
    PI * (3.0 * (self.a + self.b) - ((3.0 * self.a + self.b) * (self.a + 3.0 * self.b)).sqrt())
}
```





## 点和椭圆的关系

要判断一个点 $$(x, y)$$ 是否在椭圆上、内部或外部，可以通过将点的坐标代入椭圆的标准方程进行判断。椭圆的标准方程为：

$$ \frac{{(x - h)^2}}{{a^2}} + \frac{{(y - k)^2}}{{b^2}} = 1 $$

其中 $$(h, k)$$ 是椭圆的中心，$$a$$ 和 $$b$$ 分别是椭圆的半长轴和半短轴的长度。

点 $$(x, y)$$ 在椭圆上的条件是：

$$ \frac{{(x - h)^2}}{{a^2}} + \frac{{(y - k)^2}}{{b^2}} = 1 $$

点在椭圆内的条件是：

$$ \frac{{(x - h)^2}}{{a^2}} + \frac{{(y - k)^2}}{{b^2}} < 1 $$

点在椭圆外的条件是：

$$ \frac{{(x - h)^2}}{{a^2}} + \frac{{(y - k)^2}}{{b^2}} > 1 $$

**程序解如下**

```
// 判断点和椭圆的关系并返回枚举值
pub fn point_position(&self, x: f64, y: f64) -> PointPosition {
    let distance_squared = ((x - self.h) / self.a).powi(2) + ((y - self.k) / self.b).powi(2);

    if distance_squared == 1.0 {
        PointPosition::OnEllipse
    } else if distance_squared < 1.0 {
        PointPosition::InsideEllipse
    } else {
        PointPosition::OutsideEllipse
    }
}
```

## 焦点计算

椭圆的焦点是与椭圆定义相关的两个特殊点，这两个点对于椭圆的形状起着关键作用。椭圆焦点的坐标可以通过椭圆的参数（半长轴和半短轴的长度）计算得到。

假设椭圆的标准方程为：

$$ \frac{(x - h)^2}{a^2} + \frac{(y - k)^2}{b^2} = 1 $$

其中 $$(h, k)$$ 是椭圆的中心，$$a$$ 和 $$b$$ 分别是椭圆的半长轴和半短轴的长度。椭圆的焦点坐标 $$(F_1, F_2)$$ 可以通过以下公式计算：

$$ F_1 = (h + c, k) $$
$$ F_2 = (h - c, k) $$

其中，$$c$$ 是焦距，可以通过椭圆的长轴和短轴计算：

$$ c = \sqrt{a^2 - b^2} $$

**程序解如下**

```
// 计算椭圆的焦点坐标
pub fn calculate_foci(&self) -> ((f64, f64), (f64, f64)) {
    let c = (self.a.powi(2) - self.b.powi(2)).sqrt();
    let f1 = (self.h + c, self.k);
    let f2 = (self.h - c, self.k);

    (f1, f2)
}
```





## 拟合椭圆

椭圆的拟合是通过一组数据点找到与这些点最匹配的椭圆。这个过程通常被称为椭圆拟合或椭圆回归。下面是拟合椭圆的一般步骤，以及相关的数学公式：

### 1. 数据准备
收集一组二维数据点 $$(x_i, y_i)$$，这些点应该在平面上大致形成一个椭圆形状。

### 2. 归一化数据
为了使椭圆的拟合更加稳定，可以对数据进行归一化，将数据中心移到原点。计算数据的中心坐标 $$(\bar{x}, \bar{y})$$，然后对每个数据点进行平移变换：

$$ x_i' = x_i - \bar{x} $$
$$ y_i' = y_i - \bar{y} $$

### 3. 构建设计矩阵
对于每个归一化后的数据点 $$(x_i', y_i')$$，构建设计矩阵 $$D$$：

$$ D = \begin{bmatrix} x_1'^2 & x_1'y_1' & y_1'^2 & x_1' & y_1' \\ \vdots & \vdots & \vdots & \vdots & \vdots \\ x_n'^2 & x_n'y_n' & y_n'^2 & x_n' & y_n' \end{bmatrix} $$

### 4. 构建响应矩阵
构建响应矩阵 $$R$$，其中每个元素为 1：

$$ R = \begin{bmatrix} 1 \\ \vdots \\ 1 \end{bmatrix} $$

### 5. 解线性方程组
通过解线性方程组 $$D \cdot X = R$$，找到参数向量 $$X$$。这可以通过最小二乘法或其他方法来实现。参数向量 $$X$$ 包含了椭圆方程的系数。

### 6. 重建椭圆方程
使用参数向量 $$X$$ 中的值重建椭圆方程：

$$ ax'^2 + b x'y' + cy'^2 + dx' + ey' + f = 0 $$

其中，$$a, b, c, d, e, f$$ 是参数向量 $$X$$ 的元素。

### 7. 反归一化
如果数据进行了归一化，将椭圆方程反归一化，以获得在原始坐标系中的椭圆方程。

### 注意：
- 椭圆方程的系数 $$a, b, c, d, e, f$$ 可以与椭圆的中心、长轴和短轴之间的关系相关。
- 这个方法假设椭圆是一个标准的二次曲线。对于更一般的情况，可能需要考虑椭圆的旋转等因素。

程序解如下



```
// 拟合椭圆
fn fit(points: &[(f64, f64)]) -> Option<Ellipse> {
    let n = points.len();
    if n < 5 {
        return None; // 至少需要 5 个点
    }

    // 归一化数据
    let (mean_x, mean_y) = points.iter().fold((0.0, 0.0), |acc, &(x, y)| (acc.0 + x, acc.1 + y));
    let mean_x = mean_x / n as f64;
    let mean_y = mean_y / n as f64;

    let normalized_points: Vec<(f64, f64)> = points
        .iter()
        .map(|&(x, y)| (x - mean_x, y - mean_y))
        .collect();

    // 构建设计矩阵
    let mut d_matrix: Vec<Vec<f64>> = Vec::with_capacity(n);
    for i in 0..n {
        let (x, y) = normalized_points[i];
        d_matrix.push(vec![x * x, x * y, y * y, x, y]);
    }

    // 构建响应矩阵
    let r_matrix: Vec<Vec<f64>> = vec![vec![1.0; 1]; n];

    // 解线性方程组
    let result = gauss_elimination(&d_matrix, &r_matrix);

    match result {
        Some(parameters) => {
            // 重建椭圆方程
            let a = parameters[0];
            let b = parameters[1];
            let c = parameters[2];
            let d = parameters[3];
            let e = parameters[4];

            // 反归一化
            let h = mean_x - (2.0 * c * d + b * e) / (4.0 * a * c - b.powi(2));
            let k = mean_y - (b * d + 2.0 * a * e) / (4.0 * a * c - b.powi(2));

            let phi = 0.5 * (c - a).atan2(b);

            Some(Ellipse::new(1.0 / a, 1.0 / c, h, k, phi))
        }
        None => None,
    }
}

// 高斯消元法解线性方程组
fn gauss_elimination(a: &Vec<Vec<f64>>, b: &Vec<Vec<f64>>) -> Option<Vec<f64>> {
    let n = a.len();
    let m = a[0].len();

    let mut augmented_matrix = vec![vec![0.0; m + 1]; n];
    for i in 0..n {
        for j in 0..m {
            augmented_matrix[i][j] = a[i][j];
        }
        augmented_matrix[i][m] = b[i][0];
    }

    for i in 0..n {
        // 高斯消元
        for k in i + 1..n {
            let factor = augmented_matrix[k][i] / augmented_matrix[i][i];
            for j in i..m + 1 {
                augmented_matrix[k][j] -= factor * augmented_matrix[i][j];
            }
        }
    }

    // 回代
    let mut result = vec![0.0; n];
    for i in (0..n).rev() {
        result[i] = augmented_matrix[i][m];
        for j in i + 1..n {
            result[i] -= augmented_matrix[i][j] * result[j];
        }
        result[i] /= augmented_matrix[i][i];
    }

    Some(result)
}

```
