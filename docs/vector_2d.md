# 为了学好Rust也是拼了系列-数学库-二维向量


二维向量是数学中的一个重要概念，通常用两个实数表示。一个二维向量可以写成 $$ \mathbf{v} = \begin{bmatrix} v_1 \\ v_2 \end{bmatrix} $$，其中 $$v_1$$ 和 $$v_2$$ 是实数，分别表示向量在 x 轴和 y 轴上的分量。

二维向量可以用来表示平面上的点，其中 $$v_1$$ 表示 x 坐标，$$v_2$$ 表示 y 坐标。这样，一个点的位置可以由一个二维向量来描述。



**Rust程序定义**

```
pub struct Vector2D {
    pub x: f64,
    pub y: f64,
}
```





## 加减法

数学中的向量加法和减法是按照分量进行的。假设有两个二维向量：

$$\mathbf{v} = \begin{bmatrix} v_1 \\ v_2 \end{bmatrix}$$ 和 $$\mathbf{u} = \begin{bmatrix} u_1 \\ u_2 \end{bmatrix}$$

1. **向量加法：**
   $$\mathbf{v} + \mathbf{u} = \begin{bmatrix} v_1 + u_1 \\ v_2 + u_2 \end{bmatrix}$$

   这表示将两个向量的对应分量相加，得到一个新的向量。

2. **向量减法：**
   $$\mathbf{v} - \mathbf{u} = \begin{bmatrix} v_1 - u_1 \\ v_2 - u_2 \end{bmatrix}$$


**程序解如下**

```
// 向量加法
pub fn add(self, other: Vector2D) -> Vector2D {
    Vector2D { x: self.x + other.x, y: self.y + other.y }
}

// 向量减法
pub fn subtract(self, other: Vector2D) -> Vector2D {
    Vector2D { x: self.x - other.x, y: self.y - other.y }
}
```





## 二维向量点积（内积）

假设有两个二维向量：

$$\mathbf{v} = \begin{bmatrix} v_1 \\ v_2 \end{bmatrix}$$ 和 $$\mathbf{u} = \begin{bmatrix} u_1 \\ u_2 \end{bmatrix}$$

它们的点积（内积）定义为：

$$\mathbf{v} \cdot \mathbf{u} = v_1 \cdot u_1 + v_2 \cdot u_2$$

点积的几何意义包括计算一个向量在另一个向量方向上的投影。

**程序解如下**

```
pub fn dot_product(self, other: Vector2D) -> f64 {
    self.x * other.x + self.y * other.y
}
```

## 二维向量叉积（外积）

在二维空间中，叉积并没有直接的定义，因为它主要用于三维向量。对于二维向量 $$\mathbf{v} = \begin{bmatrix} v_1 \\ v_2 \end{bmatrix}$$ 和 $$\mathbf{u} = \begin{bmatrix} u_1 \\ u_2 \end{bmatrix}$$，它们的"叉积"可以通过以下公式表示：

$$\mathbf{v} \times \mathbf{u} = v_1 \cdot u_2 - v_2 \cdot u_1$$

在这个情况下，它的结果是一个标量而不是一个向量，这代表了两个向量所在平面的有向面积。这个值可以用来判断两个向量的相对方向（顺时针还是逆时针）和计算面积。

请注意，在二维空间中，叉积的应用相对有限，主要因为它返回一个标量而不是一个向量。在三维空间中，叉积更加常见，因为它返回垂直于原始两个向量所在平面的向量。

**程序解如下**

```
// 向量叉积
pub fn cross_product(self, other: Vector2D) -> f64 {
    self.x * other.y - self.y * other.x
}
```
