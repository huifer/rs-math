# 为了学好Rust也是拼了系列-数学库-矩形

在几何学的世界中，矩形是一种引人注目的简洁而实用的几何形状。与圆形相比，矩形展现出其独特的几何特性和广泛的应用领域。其定义直截了当：矩形是一个拥有四个直角和相对边相等的四边形。这使得矩形在建筑、工程和设计中成为不可或缺的基本元素。


## 矩形的定义

矩形的定义同样简单而直截了当，它是平面上由两条相对且平行的边界线所围成的四边形。与圆形不同，矩形的角是直角，其四个角分别为90度。

在数学上，我们可以用坐标系和数学公式来描述矩形。假设矩形的两个对角顶点分别为$(x_1, y_1)$和$(x_2, y_2)$，其中$x_1, y_1, x_2, y_2$是相应顶点的坐标。那么，矩形的定义可以表示为：

$$
\begin{align*}
x_1 &\leq x \leq x_2 \\
y_1 &\leq y \leq y_2
\end{align*}
$$

用 Rust 定义矩形可以使用如下程序


```rust
#[derive(Debug, PartialEq)]
pub struct Rectangle {
    pub x1: f64,
    pub y1: f64,
    pub x2: f64,
    pub y2: f64,
}
```

其中，$(x_1, y_1)$是矩形的左上角顶点的坐标，$(x_2, y_2)$是矩形的右下角顶点的坐标。





## 计算矩形

### 已知XY和长宽求矩形



已知一个点以及矩形的宽度和高度时，可以通过数学逻辑来解释确定矩形的四个顶点的过程。

假设矩形的左上角坐标为 $$(x_1, y_1)$$，宽度为 $$w$$，高度为 $$h$$。我们有一个点 $$(x, y)$$。

1. **左上角情况：**
   - 如果 $$x$$ 在矩形的右边缘之外（即 $$x > x_1 + w$$），且 $$y$$ 在矩形的下边缘之外（即 $$y > y_1 + h$$），那么 $$(x, y)$$ 是矩形的左上角。

2. **左下角情况：**
   - 如果 $$x$$ 在矩形的右边缘之外（即 $$x > x_1 + w$$），且 $$y$$ 在矩形的上边缘之外（即 $$y < y_1$$），那么 $$(x, y)$$ 是矩形的左下角。

3. **右上角情况：**
   - 如果 $$x$$ 在矩形的左边缘之外（即 $$x < x_1$$），且 $$y$$ 在矩形的下边缘之外（即 $$y > y_1 + h$$），那么 $$(x, y)$$ 是矩形的右上角。

4. **右下角情况：**
   - 如果 $$x$$ 在矩形的左边缘之外（即 $$x < x_1$$），且 $$y$$ 在矩形的上边缘之外（即 $$y < y_1$$），那么 $$(x, y)$$ 是矩形的右下角。

这些条件确保了点 $$(x, y)$$ 与矩形的相对位置关系，使得我们能够根据这个点和矩形的宽度和高度来确定矩形的四个顶点坐标。

**程序解如下**

```
///
pub fn new_from_corner(x: f64, y: f64, width: f64, height: f64) -> Self {
    match (x, y) {
        (x1, y1) if x1 + width <= x && y1 + height <= y => {
            // 左上角
            Rectangle {
                x1: x - width,
                y1: y - height,
                x2: x,
                y2: y,
            }
        }
        (x1, y1) if x1 <= x && y1 <= y => {
            // 左下角
            Rectangle {
                x1: x1,
                y1: y1 - height,
                x2: x1 + width,
                y2: y,
            }
        }
        (x1, y1) if x1 + width >= x && y1 <= y => {
            // 右上角
            Rectangle {
                x1: x,
                y1: y1 - height,
                x2: x + width,
                y2: y,
            }
        }
        (x1, y1) if x1 <= x && y1 + height >= y => {
            // 右下角
            Rectangle {
                x1: x1,
                y1: y,
                x2: x1 + width,
                y2: y + height,
            }
        }
        _ => panic!("Invalid corner point"),
    }
}
```

## 矩形的缩放

缩放矩形可以通过改变其左上角和右下角顶点的坐标来实现。设原始矩形的左上角顶点坐标为 $$(x_1, y_1)$$，右下角顶点坐标为 $$(x_2, y_2)$$。如果要对矩形进行水平和垂直方向的缩放，可以分别对 $$x$$ 和 $$y$$ 方向上的坐标进行缩放。

1. **水平缩放：**
   水平方向上的缩放因子为 $$s_x$$，则新的右下角顶点的 $$x$$ 坐标为 $$x_1 + s_x \cdot (x_2 - x_1)$$。左上角的 $$x$$ 坐标保持不变。

2. **垂直缩放：**
   垂直方向上的缩放因子为 $$s_y$$，则新的右下角顶点的 $$y$$ 坐标为 $$y_1 + s_y \cdot (y_2 - y_1)$$。左上角的 $$y$$ 坐标保持不变。

综合起来，缩放后的矩形的新右下角顶点坐标为：
$$
\begin{align*}
x_2' &= x_1 + s_x \cdot (x_2 - x_1) \\
y_2' &= y_1 + s_y \cdot (y_2 - y_1)
\end{align*}
$$

**程序解如下**

```
/// 缩放矩形
pub fn scale(&mut self, sx: f64, sy: f64) {
    // 计算新的右下角顶点坐标
    self.x2 = self.x1 + sx * (self.x2 - self.x1);
    self.y2 = self.y1 + sy * (self.y2 - self.y1);
}
```





## 矩形的旋转



### 绕原点

要旋转一个矩形，可以使用旋转矩阵的概念。旋转矩阵可以用来对一个点或一组点进行旋转。对于一个二维平面上的点 $$(x, y)$$，绕原点逆时针旋转角度 $$\theta$$ 的新坐标 $$(x', y')$$ 可以通过以下公式得到：

$$
\begin{align*}
x' &= x \cdot \cos(\theta) - y \cdot \sin(\theta) \\
y' &= x \cdot \sin(\theta) + y \cdot \cos(\theta)
\end{align*}
$$

对于矩形来说，可以分别对矩形的四个顶点进行旋转操作。假设矩形的左上角顶点坐标为 $$(x_1, y_1)$$，右下角顶点坐标为 $$(x_2, y_2)$$，则旋转后的坐标为：

$$
\begin{align*}
x_1' &= x_1 \cdot \cos(\theta) - y_1 \cdot \sin(\theta) \\
y_1' &= x_1 \cdot \sin(\theta) + y_1 \cdot \cos(\theta) \\
x_2' &= x_2 \cdot \cos(\theta) - y_2 \cdot \sin(\theta) \\
y_2' &= x_2 \cdot \sin(\theta) + y_2 \cdot \cos(\theta)
\end{align*}
$$

程序解如下

```
/// 旋转一个矩形（绕原点）
pub fn rotate(&mut self, angle: f64)->Rectangle {
    // 计算旋转后的左上角顶点坐标
    let new_x1 = self.x1 * angle.cos() - self.y1 * angle.sin();
    let new_y1 = self.x1 * angle.sin() + self.y1 * angle.cos();

    // 计算旋转后的右下角顶点坐标
    let new_x2 = self.x2 * angle.cos() - self.y2 * angle.sin();
    let new_y2 = self.x2 * angle.sin() + self.y2 * angle.cos();

    Rectangle {
        x1: new_x1,
        y1: new_y1,
        x2: new_x2,
        y2: new_y2,
    }
}
```



### 绕任意点

如果要让矩形绕平面上的任意一点 $$(x_{\text{rot}}, y_{\text{rot}})$$ 旋转，可以按照以下步骤进行：

1. 计算矩形中心点相对于旋转点的坐标：
   $$
   \begin{align*}
   x_c' &= x_c - x_{\text{rot}} \\
   y_c' &= y_c - y_{\text{rot}}
   \end{align*}
   $$
   其中，$$x_c$$ 和 $$y_c$$ 是矩形的中心点坐标。

2. 使用旋转矩阵对矩形进行旋转：
   $$
   \begin{align*}
   x_1'' &= (x_1 - x_{\text{rot}}) \cos(\theta) - (y_1 - y_{\text{rot}}) \sin(\theta) \\
   y_1'' &= (x_1 - x_{\text{rot}}) \sin(\theta) + (y_1 - y_{\text{rot}}) \cos(\theta) \\
   x_2'' &= (x_2 - x_{\text{rot}}) \cos(\theta) - (y_2 - y_{\text{rot}}) \sin(\theta) \\
   y_2'' &= (x_2 - x_{\text{rot}}) \sin(\theta) + (y_2 - y_{\text{rot}}) \cos(\theta)
   \end{align*}
   $$

3. 将旋转后的中心点坐标平移到旋转点：
   $$
   \begin{align*}
   x_c &= x_1'' + x_{\text{rot}} \\
   y_c &= y_1'' + y_{\text{rot}}
   \end{align*}
   $$

经过上述操作可以实现矩形绕平面上的任意点进行旋转。这个过程保持了选择的旋转点不变，只是对矩形进行了旋转。



**程序解如下**

```
/// 根据点旋转
pub fn rotate_around_point(&self, angle: f64, x_rot: f64, y_rot: f64) -> Rectangle {
    // 计算中心点坐标
    let x_c = (self.x1 + self.x2) / 2.0;
    let y_c = (self.y1 + self.y2) / 2.0;

    // 计算中心点相对于旋转点的坐标
    let x_c_prime = x_c - x_rot;
    let y_c_prime = y_c - y_rot;

    // 使用旋转矩阵对矩形进行旋转
    let x1_prime = (self.x1 - x_rot) * angle.cos() - (self.y1 - y_rot) * angle.sin();
    let y1_prime = (self.x1 - x_rot) * angle.sin() + (self.y1 - y_rot) * angle.cos();
    let x2_prime = (self.x2 - x_rot) * angle.cos() - (self.y2 - y_rot) * angle.sin();
    let y2_prime = (self.x2 - x_rot) * angle.sin() + (self.y2 - y_rot) * angle.cos();

    // 将旋转后的中心点坐标平移到旋转点
    let x1 = x1_prime + x_rot;
    let y1 = y1_prime + y_rot;
    let x2 = x2_prime + x_rot;
    let y2 = y2_prime + y_rot;

    Rectangle { x1, y1, x2, y2 }
}
```





## 矩形的关系



### 判断点是否在矩形内

要判断一个点 $$(x, y)$$ 是否在矩形内，可以使用矩形的边界信息进行判断。矩形由左上角顶点 $$(x_1, y_1)$$ 和右下角顶点 $$(x_2, y_2)$$ 定义。

点 $$(x, y)$$ 在矩形内的条件为：

$$ x_1 \leq x \leq x_2 \quad \text{且} \quad y_1 \leq y \leq y_2 $$

如果这个条件满足，那么点 $$(x, y)$$ 就在矩形内。这是因为 $$x$$ 坐标必须在矩形左上角和右下角的 $$x$$ 范围内，而 $$y$$ 坐标必须在矩形左上角和右下角的 $$y$$ 范围内。

**程序解如下**

```
/// 判断点是否在矩形内
pub fn point_inside(&self, x: f64, y: f64) -> bool {
    x >= self.x1 && x <= self.x2 && y >= self.y1 && y <= self.y2
}
```



### **判断两个矩形是否相交**

两个矩形相交的条件是，其中一个矩形的右下角顶点的横坐标大于另一个矩形的左上角顶点的横坐标，且一个矩形的左上角顶点的横坐标小于另一个矩形的右下角顶点的横坐标。同时，一个矩形的右下角顶点的纵坐标大于另一个矩形的左上角顶点的纵坐标，且一个矩形的左上角顶点的纵坐标小于另一个矩形的右下角顶点的纵坐标。

以数学公式表示，两个矩形 $$(x_1, y_1, x_2, y_2)$$ 和 $$(x_3, y_3, x_4, y_4)$$ 相交的条件为：

$$ x_1 \leq x_4 \quad \text{且} \quad x_2 \geq x_3 \quad \text{且} \quad y_1 \leq y_4 \quad \text{且} \quad y_2 \geq y_3 $$

**程序解如下**

```
/// 判断两个矩形是否相交
pub fn intersect(&self, other: &Rectangle) -> bool {
    self.x1 <= other.x2 && self.x2 >= other.x1 && self.y1 <= other.y2 && self.y2 >= other.y1
}
```



### 判断矩形是否包含矩形

要判断一个矩形是否包含另一个矩形，我们可以检查第一个矩形的边界是否完全包含了第二个矩形。具体而言，对于两个矩形 $$(x_1, y_1, x_2, y_2)$$ 和 $$(x_3, y_3, x_4, y_4)$$，第一个矩形包含第二个矩形的条件为：

$$ x_1 \leq x_3 \quad \text{且} \quad x_2 \geq x_4 \quad \text{且} \quad y_1 \leq y_3 \quad \text{且} \quad y_2 \geq y_4 $$

**程序解如下**
