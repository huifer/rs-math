# 为了学好Rust也是拼了系列-数学库-三维维向量

三维向量是指具有三个有序数值组成的向量，通常表示为$$ \mathbf{v} = \begin{bmatrix} x \\ y \\ z \end{bmatrix} $$。其中，$$x$$、$$y$$和$$z$$分别表示该向量在三个坐标轴上的分量。

在数学中，我们可以使用线性代数的概念来定义三维向量。一个三维向量可以被看作是三个标量的有序组合，这些标量分别表示在三个坐标轴上的投影。具体地，给定一个三维向量 $$\mathbf{v} = \begin{bmatrix} x \\ y \\ z \end{bmatrix}$$，我们可以定义它为：

$$\mathbf{v} = x\mathbf{i} + y\mathbf{j} + z\mathbf{k}$$

其中，$$\mathbf{i}$$、$$\mathbf{j}$$和$$\mathbf{k}$$分别是三维空间中的标准基向量。它们的定义如下：

$$\mathbf{i} = \begin{bmatrix} 1 \\ 0 \\ 0 \end{bmatrix}, \quad \mathbf{j} = \begin{bmatrix} 0 \\ 1 \\ 0 \end{bmatrix}, \quad \mathbf{k} = \begin{bmatrix} 0 \\ 0 \\ 1 \end{bmatrix}$$

这样，三维向量 $$\mathbf{v}$$ 就可以通过在每个基向量上的投影（乘以相应的标量）来表示。这种表示方式有助于理解向量在三维空间中的方向和长度。

**程序定义如下**

```
#[derive(Debug)]
pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
```





## 加减法

三维向量的加法和减法遵循分量相加和相减的原则。考虑两个三维向量：

$$\mathbf{v}_1 = \begin{bmatrix} x_1 \\ y_1 \\ z_1 \end{bmatrix}$$ 和 $$\mathbf{v}_2 = \begin{bmatrix} x_2 \\ y_2 \\ z_2 \end{bmatrix}$$

它们的加法定义为：

$$\mathbf{v}_1 + \mathbf{v}_2 = \begin{bmatrix} x_1 + x_2 \\ y_1 + y_2 \\ z_1 + z_2 \end{bmatrix}$$

这就是将两个向量的相应分量相加得到的新向量。同样，两个向量的减法定义为：

$$\mathbf{v}_1 - \mathbf{v}_2 = \begin{bmatrix} x_1 - x_2 \\ y_1 - y_2 \\ z_1 - z_2 \end{bmatrix}$$

这就是将两个向量的相应分量相减得到的新向量。



程序解如下



```
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
```

## 数乘

数量乘法是指将一个标量与一个向量的每个分量相乘的操作。给定一个标量 $$c$$ 和三维向量 $$\mathbf{v} = \begin{bmatrix} x \\ y \\ z \end{bmatrix}$$，数量乘法的定义如下：

$$ c \cdot \mathbf{v} = \begin{bmatrix} c \cdot x \\ c \cdot y \\ c \cdot z \end{bmatrix} $$

在这个操作中，标量 $$c$$ 与向量 $$\mathbf{v}$$ 的每个分量相乘，产生一个新的向量。这个新向量的每个分量都是原向量相应分量与标量的乘积。

数量乘法的几何解释是对向量进行缩放或拉伸。如果 $$c > 1$$，那么结果向量的长度会增加；如果 $$0 < c < 1$$，那么结果向量的长度会减小；如果 $$c < 0$$，那么结果向量方向会反转，并且长度的绝对值会增加。当 $$c = 1$$ 时，向量保持不变；当 $$c = 0$$ 时，结果向量的所有分量都为零向量。

例如，如果有向量 $$\mathbf{v} = \begin{bmatrix} 2 \\ 3 \\ -1 \end{bmatrix}$$ 和标量 $$c = 3$$，则数量乘法的结果为：

$$ 3 \cdot \mathbf{v} = \begin{bmatrix} 3 \cdot 2 \\ 3 \cdot 3 \\ 3 \cdot (-1) \end{bmatrix} = \begin{bmatrix} 6 \\ 9 \\ -3 \end{bmatrix} $$

这表示将向量 $$\mathbf{v}$$ 中的每个分量都乘以标量 $$3$$，得到一个新的向量。



**程序解如下**

```
// 数量乘法
pub fn scalar_multiply(&self, scalar: f64) -> Vector3D {
    Vector3D {
        x: self.x * scalar,
        y: self.y * scalar,
        z: self.z * scalar,
    }
}
```

## 叉积



向量乘法中的叉积（cross product）是两个向量之间的一种特殊的乘法运算。给定两个三维向量 $$\mathbf{v}_1 = \begin{bmatrix} x_1 \\ y_1 \\ z_1 \end{bmatrix}$$ 和 $$\mathbf{v}_2 = \begin{bmatrix} x_2 \\ y_2 \\ z_2 \end{bmatrix}$$，它们的叉积 $$\mathbf{v}_1 \times \mathbf{v}_2$$ 定义如下：

$$\mathbf{v}_1 \times \mathbf{v}_2 = \begin{bmatrix} y_1z_2 - z_1y_2 \\ z_1x_2 - x_1z_2 \\ x_1y_2 - y_1x_2 \end{bmatrix}$$



**程序解如下**

```
// 计算叉积
pub fn cross_product(&self, other: &Vector3D) -> Vector3D {
    Vector3D {
        x: self.y * other.z - self.z * other.y,
        y: self.z * other.x - self.x * other.z,
        z: self.x * other.y - self.y * other.x,
    }
}
```



## 模长（长度）

模长，也称为长度、幅值或矢量的大小，是指一个向量在空间中的长度。对于三维向量 $$\mathbf{v} = \begin{bmatrix} x \\ y \\ z \end{bmatrix}$$，它的模长计算公式为：

$$ |\mathbf{v}| = \sqrt{x^2 + y^2 + z^2} $$

这个公式来源于三维空间中的勾股定理。模长表示从原点到向量终点的距离，或者说向量的大小。

**程序解如下**

```
// 计算模长
pub fn magnitude(&self) -> f64 {
    (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
}
```





## 夹角

计算两个三维向量之间的夹角通常可以使用向量的点积（内积）和反余弦函数。假设有两个向量 $$\mathbf{v}_1$$ 和 $$\mathbf{v}_2$$，它们的夹角为 $$\theta$$，则夹角的计算公式为：

$$ \cos(\theta) = \frac{\mathbf{v}_1 \cdot \mathbf{v}_2}{|\mathbf{v}_1| \cdot |\mathbf{v}_2|} $$

其中，$$\cdot$$ 表示点积，$$ |\mathbf{v}| $$ 表示向量的模长。夹角 $$\theta$$ 可以通过反余弦函数求得：

$$ \theta = \cos^{-1}\left(\frac{\mathbf{v}_1 \cdot \mathbf{v}_2}{|\mathbf{v}_1| \cdot |\mathbf{v}_2|}\right) $$



**程序解如下**

```
// 计算两个向量之间的夹角（弧度）
pub fn angle_between(&self, other: &Vector3D) -> f64 {
    let dot_product = self.dot_product(other);
    let magnitude_product = self.magnitude() * other.magnitude();

    dot_product.acos() / magnitude_product
}
```
