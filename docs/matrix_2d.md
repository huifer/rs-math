# 为了学好Rust也是拼了系列-数学库-二维矩阵



二维矩阵是数学中的一个重要概念，它是由元素排列成行和列的矩形数组。通常用大写字母表示矩阵，例如 $$A$$，矩阵中的元素用小写字母表示，例如 $$a_{ij}$$，其中 $$i$$ 表示行索引，$$j$$ 表示列索引。

一个二维矩阵 $$A$$ 可以表示为：

$$ A = \begin{bmatrix} a_{11} & a_{12} & \cdots & a_{1n} \\ a_{21} & a_{22} & \cdots & a_{2n} \\ \vdots & \vdots & \ddots & \vdots \\ a_{m1} & a_{m2} & \cdots & a_{mn} \end{bmatrix} $$

这里，$$m$$ 是矩阵的行数，$$n$$ 是矩阵的列数。矩阵的大小通常以 "行数 × 列数" 的形式表示。

例如，一个 2x3 的矩阵可以写作：

$$ B = \begin{bmatrix} b_{11} & b_{12} & b_{13} \\ b_{21} & b_{22} & b_{23} \end{bmatrix} $$

其中，这个矩阵有两行（$$m=2$$）和三列（$$n=3$$）。

程序定义如下

```
pub struct Matrix {
    rows: usize,    // 行数
    cols: usize,    // 列数
    data: Vec<Vec<f64>>, // 存储矩阵元素的二维向量
}
```





## 矩阵加减法

矩阵的加法和减法是基本的线性代数运算，它们分别在相同位置对应元素进行加法和减法。以下是矩阵加法和减法的数学定义：

### 矩阵加法：

给定两个相同大小的矩阵 $$A$$ 和 $$B$$，它们的和 $$C$$（记作 $$C = A + B$$）的每个元素 $$c_{ij}$$ 都是对应位置元素的和：

$$ C = A + B $$
$$ c_{ij} = a_{ij} + b_{ij} $$

其中，$$a_{ij}$$ 是矩阵 $$A$$ 中第 $$i$$ 行第 $$j$$ 列的元素，$$b_{ij}$$ 是矩阵 $$B$$ 中相应位置的元素。

### 矩阵减法：

给定两个相同大小的矩阵 $$A$$ 和 $$B$$，它们的差 $$D$$（记作 $$D = A - B$$）的每个元素 $$d_{ij}$$ 都是对应位置元素的差：

$$ D = A - B $$
$$ d_{ij} = a_{ij} - b_{ij} $$

同样，其中 $$a_{ij}$$ 是矩阵 $$A$$ 中第 $$i$$ 行第 $$j$$ 列的元素，$$b_{ij}$$ 是矩阵 $$B$$ 中相应位置的元素。

>    注意：矩阵加法和减法要求两个矩阵的维度必须相同，即它们必须有相同的行数和列数。只有在这种情况下，对应位置的元素才能进行加法或减法运算。

**程序解如下**

```
// 矩阵加法
pub fn add(&self, other: &Matrix) -> Option<Matrix> {
    if self.rows == other.rows && self.cols == other.cols {
        let mut result_data = Vec::with_capacity(self.rows);
        for i in 0..self.rows {
            let mut row = Vec::with_capacity(self.cols);
            for j in 0..self.cols {
                row.push(self.data[i][j] + other.data[i][j]);
            }
            result_data.push(row);
        }
        Some(Matrix {
            rows: self.rows,
            cols: self.cols,
            data: result_data,
        })
    } else {
        None
    }
}

// 矩阵减法
pub fn subtract(&self, other: &Matrix) -> Option<Matrix> {
    if self.rows == other.rows && self.cols == other.cols {
        let mut result_data = Vec::with_capacity(self.rows);
        for i in 0..self.rows {
            let mut row = Vec::with_capacity(self.cols);
            for j in 0..self.cols {
                row.push(self.data[i][j] - other.data[i][j]);
            }
            result_data.push(row);
        }
        Some(Matrix {
            rows: self.rows,
            cols: self.cols,
            data: result_data,
        })
    } else {
        None
    }
}
```



## 矩阵的乘法

矩阵乘法是一个涉及行和列的复杂运算。给定两个矩阵 $$A$$ 和 $$B$$，它们的乘积 $$C$$（记作 $$C = A \cdot B$$）的定义如下：

假设矩阵 $$A$$ 的维度为 $$m \times n$$，矩阵 $$B$$ 的维度为 $$n \times p$$，那么矩阵 $$C$$ 的维度为 $$m \times p$$。

矩阵 $$C$$ 中的每个元素 $$c_{ij}$$ 计算方式如下：

$$ c_{ij} = \sum_{k=1}^{n} a_{ik} \cdot b_{kj} $$

其中，$$a_{ik}$$ 是矩阵 $$A$$ 中第 $$i$$ 行第 $$k$$ 列的元素，$$b_{kj}$$ 是矩阵 $$B$$ 中第 $$k$$ 行第 $$j$$ 列的元素。这个计算过程是对矩阵 $$A$$ 的第 $$i$$ 行和矩阵 $$B$$ 的第 $$j$$ 列进行点积（内积），得到矩阵 $$C$$ 中元素的值。

矩阵乘法的关键要点：
1. 乘法的前提是第一个矩阵的列数等于第二个矩阵的行数。
2. 乘积矩阵的行数等于第一个矩阵的行数，列数等于第二个矩阵的列数。
3. 矩阵乘法不满足交换律，即一般情况下 $$A \cdot B \neq B \cdot A$$。



**程序解如下**

```
/// 矩阵的乘法
pub fn multiply(&self, other: &Matrix) -> Option<Matrix> {
    // 检查矩阵维度是否允许相乘
    if self.cols != other.rows {
        return None;
    }

    let mut result_data = Vec::with_capacity(self.rows);
    for i in 0..self.rows {
        let mut row = Vec::with_capacity(other.cols);
        for j in 0..other.cols {
            let mut sum = 0.0;
            for k in 0..self.cols {
                sum += self.data[i][k] * other.data[k][j];
            }
            row.push(sum);
        }
        result_data.push(row);
    }

    Some(Matrix {
        rows: self.rows,
        cols: other.cols,
        data: result_data,
    })
}
```





## 转置

矩阵的转置是一种操作，通过这种操作，矩阵的行和列交换位置。给定一个矩阵 $$A$$，其转置记作 $$A^T$$。对于矩阵 $$A$$ 中的每一个元素 $$a_{ij}$$，在转置后的矩阵 $$A^T$$ 中，该元素位于 $$A$$ 中的列 $$j$$ 和行 $$i$$ 的位置。

具体来说，如果矩阵 $$A$$ 的维度是 $$m \times n$$，那么它的转置 $$A^T$$ 的维度是 $$n \times m$$。矩阵转置的数学表示如下：

$$ (A^T)_{ij} = A_{ji} $$

其中，$$(A^T)_{ij}$$ 表示转置矩阵 $$A^T$$ 中的第 $$i$$ 行第 $$j$$ 列的元素，而 $$A_{ji}$$ 表示原矩阵 $$A$$ 中的第 $$j$$ 行第 $$i$$ 列的元素。

矩阵转置的性质：
1. $$(A^T)^T = A$$，即对一个矩阵进行两次转置等于原矩阵。
2. $$(A + B)^T = A^T + B^T$$，即转置的和等于和的转置。
3. $$(kA)^T = kA^T$$，其中 $$k$$ 是常数。



下面是一个简单的例子，演示了如何计算矩阵的转置：

假设有矩阵 $$A$$：

$$ A = \begin{bmatrix} 1 & 2 & 3 \\ 4 & 5 & 6 \end{bmatrix} $$

那么，矩阵 $$A$$ 的转置 $$A^T$$ 是：

$$ A^T = \begin{bmatrix} 1 & 4 \\ 2 & 5 \\ 3 & 6 \end{bmatrix} $$

你可以看到，原矩阵 $$A$$ 的行变成了转置矩阵 $$A^T$$ 的列，而原矩阵 $$A$$ 的列变成了转置矩阵 $$A^T$$ 的行。



**程序解如下**

```
/// 转置矩阵
pub fn transpose(&self) -> Matrix {
    let mut result_data = Vec::with_capacity(self.cols);
    for j in 0..self.cols {
        let mut row = Vec::with_capacity(self.rows);
        for i in 0..self.rows {
            row.push(self.data[i][j]);
        }
        result_data.push(row);
    }

    Matrix {
        rows: self.cols,
        cols: self.rows,
        data: result_data,
    }
}
```

## 行列式

行列式是一个与方阵相关的数学概念，它为方阵提供了一个标量值。给定一个 $$n \times n$$ 的方阵 $$A$$，其行列式通常表示为 $$|A|$$ 或 $$\text{det}(A)$$。

行列式的计算涉及矩阵的元素，其表达式如下：

$$ |A| = \sum_{\sigma \in S_n} \text{sgn}(\sigma) \cdot a_{1\sigma(1)} \cdot a_{2\sigma(2)} \cdot \ldots \cdot a_{n\sigma(n)} $$

这里，$$S_n$$ 是所有 $$n$$ 阶排列的集合，$$\sigma$$ 是一个排列，$$\text{sgn}(\sigma)$$ 是排列的符号（奇置换为 -1，偶置换为 1），$$a_{i\sigma(i)}$$ 是矩阵 $$A$$ 中的元素。

这个公式展示了一个递归的计算过程，它对所有可能的排列进行了求和。对于每个排列，我们将相应位置的元素相乘，然后乘以符号 $$\text{sgn}(\sigma)$$。最后，将所有这些结果相加得到最终的行列式值。

行列式有一些重要的性质：
1. 行列式的值与矩阵的行列互换无关，即 $$|A| = |A^T|$$。
2. 如果矩阵有两行或两列相同，则行列式为零。
3. 行列式的值等于它的伴随矩阵（adjugate matrix）与原矩阵的乘积。

**程序解如下**

```
// 计算矩阵的行列式
    pub fn determinant(&self) -> Option<f64> {
        // 检查矩阵是否为方阵
        if self.rows != self.cols {
            return None;
        }

        // 递归计算行列式
        self.calculate_determinant(&self.data)
    }

    // 递归计算行列式的辅助函数
    fn calculate_determinant(&self, matrix: &Vec<Vec<f64>>) -> Option<f64> {
        match self.rows {
            1 => Some(matrix[0][0]), // 1x1 矩阵的行列式为其唯一元素的值
            2 => Some(matrix[0][0] * matrix[1][1] - matrix[0][1] * matrix[1][0]), // 2x2 矩阵的行列式计算公式
            _ => {
                let mut det = 0.0;

                for col in 0..self.cols {
                    // 计算代数余子式
                    let submatrix_data: Vec<Vec<f64>> = matrix
                        .iter()
                        .enumerate()
                        .filter(|&(i, _)| i != 0)
                        .map(|(_, row)| row.iter().enumerate().filter(|&(j, _)| j != col).map(|(_, &val)| val).collect())
                        .collect();

                    let submatrix = Matrix {
                        rows: self.rows - 1,
                        cols: self.cols - 1,
                        data: submatrix_data,
                    };

                    let cofactor = if col % 2 == 0 { 1.0 } else { -1.0 };

                    match self.calculate_determinant(&submatrix.data) {
                        Some(sub_det) => det += cofactor * matrix[0][col] * sub_det,
                        None => return None,
                    }
                }

                Some(det)
            }
        }
    }
```





## 逆矩阵

逆矩阵是与原矩阵相乘后得到单位矩阵的矩阵。给定一个方阵 $$A$$，如果存在一个方阵 $$B$$ 使得 $$AB = BA = I$$，其中 $$I$$ 是单位矩阵，那么矩阵 $$B$$ 就是矩阵 $$A$$ 的逆矩阵，通常表示为 $$A^{-1}$$。

逆矩阵的性质：
1. 只有方阵才有可能有逆矩阵。
2. 如果矩阵 $$A$$ 有逆矩阵，则逆矩阵唯一。

逆矩阵的计算方法：
如果一个 $$n \times n$$ 的矩阵 $$A$$ 可逆，其逆矩阵 $$A^{-1}$$ 可以通过以下公式计算：

$$ A^{-1} = \frac{1}{\text{det}(A)} \cdot \text{adj}(A) $$

其中，$$\text{det}(A)$$ 是矩阵 $$A$$ 的行列式，$$\text{adj}(A)$$ 是矩阵 $$A$$ 的伴随矩阵。伴随矩阵的元素由原矩阵的代数余子式组成，并按矩阵的转置排列。

具体而言，对于 $$n \times n$$ 矩阵 $$A$$，其伴随矩阵的元素 $$(\text{adj}(A))_{ij}$$ 可以通过以下方式计算：

$$ (\text{adj}(A))_{ij} = (-1)^{i+j} \cdot \text{det}(M_{ij}) $$

其中，$$M_{ij}$$ 是去掉矩阵 $$A$$ 的第 $$i$$ 行和第 $$j$$ 列后的子矩阵。



## 特征值和特征向量



特征值和特征向量是矩阵理论中的重要概念，它们在许多数学和工程应用中起着关键的作用。

### 特征值和特征向量的定义

给定一个方阵 $$A$$，非零向量 $$v$$ 是 $$A$$ 的特征向量，如果存在一个标量 $$\lambda$$，使得满足如下关系：

$$ Av = \lambda v $$

这里，$$\lambda$$ 是 $$A$$ 的特征值，$$v$$ 是对应于特征值 $$\lambda$$ 的特征向量。

### 解释和性质

1. **特征值（Eigenvalues）：** 特征值是一个标量，它表示了矩阵 $$A$$ 在变换中的缩放因子。一个 $$n \times n$$ 的矩阵 $$A$$ 最多有 $$n$$ 个特征值。

2. **特征向量（Eigenvectors）：** 特征向量是与特征值关联的非零向量，其在矩阵变换下只发生缩放而不改变方向。特征向量的长度（模）不是固定的，其方向是关键的。

3. **特征值方程（Characteristic Equation）：** 特征值和特征向量的关系由特征值方程表示：

$$ \text{det}(A - \lambda I) = 0 $$

其中，$$\text{det}$$ 表示矩阵的行列式，$$I$$ 是单位矩阵。特征值是满足方程的根。

4. **特征空间：** 与特定特征值关联的所有特征向量形成一个向量空间，称为特征空间。

5. **线性独立性：** 不同特征值对应的特征向量是线性独立的，即它们不可由其他特征向量的线性组合表示。

6. **对角化：** 如果矩阵 $$A$$ 有 $$n$$ 个线性独立的特征向量，可以构成一个特征向量矩阵 $$V$$，则 $$A$$ 可对角化为 $$V^{-1}AV = \Lambda$$，其中 $$\Lambda$$ 是一个对角矩阵，其对角线上的元素是 $$A$$ 的特征值。

**程序解如下**

```
// 计算矩阵的特征值和特征向量
pub fn eigenvalue_eigenvector(&self) -> Option<(Vec<f64>, Vec<Vec<f64>>)> {
    // 检查矩阵是否为方阵
    if self.rows != self.cols {
        return None;
    }

    // 构造特征值方程的左侧矩阵 A - λI
    let a_minus_lambda_i: Matrix = self.subtract_identity();

    // 求解特征值
    let eigenvalues = match a_minus_lambda_i.determinant() {
        Some(det) => self.find_eigenvalues(det),
        None => return None,
    };

    // 求解特征向量
    let mut eigenvectors: Vec<Vec<f64>> = Vec::with_capacity(eigenvalues.len());
    for &eigenvalue in &eigenvalues {
        let eigenvector = self.solve_eigenvector(eigenvalue);
        eigenvectors.push(eigenvector);
    }

    Some((eigenvalues, eigenvectors))
}

// 辅助方法：构造 A - λI
fn subtract_identity(&self) -> Matrix {
    let mut result_data = Vec::with_capacity(self.rows);
    for i in 0..self.rows {
        let mut row = Vec::with_capacity(self.cols);
        for j in 0..self.cols {
            let element = if i == j {
                self.data[i][j] - 1.0 // λ 对应的位置减去 1
            } else {
                self.data[i][j]
            };
            row.push(element);
        }
        result_data.push(row);
    }

    Matrix {
        rows: self.rows,
        cols: self.cols,
        data: result_data,
    }
}

// 辅助方法：求解特征值方程的根
fn find_eigenvalues(&self, det_a_minus_lambda_i: f64) -> Vec<f64> {
    // 这里可以使用任何你喜欢的求根算法，这里简化为一个简单的方式
    // 注意：这里没有处理重根的情况，实际应用中可能需要更复杂的算法
    let mut eigenvalues = Vec::with_capacity(self.rows);
    for i in 0..self.rows {
        eigenvalues.push(self.data[i][i]);
    }
    eigenvalues
}

// 辅助方法：求解特征向量
fn solve_eigenvector(&self, eigenvalue: f64) -> Vec<f64> {
    // 这里可以使用任何你喜欢的求解线性方程组的算法，这里简化为一个简单的方式
    // 注意：这里没有处理奇异矩阵的情况，实际应用中可能需要更复杂的算法
    let mut augmented_matrix_data = Vec::with_capacity(self.rows);
    for i in 0..self.rows {
        let mut row = Vec::with_capacity(self.cols + 1);
        for j in 0..self.cols {
            row.push(self.data[i][j] - eigenvalue * if i == j { 1.0 } else { 0.0 });
        }
        row.push(0.0); // 右侧的常数项
        augmented_matrix_data.push(row);
    }

    let mut  augmented_matrix = Matrix {
        rows: self.rows,
        cols: self.cols + 1,
        data: augmented_matrix_data,
    };

    // 简单的高斯消元法求解线性方程组
    let mut solution = Vec::with_capacity(self.rows);
    for i in 0..self.rows {
        let mut pivot_row = i;
        for j in i + 1..self.rows {
            if augmented_matrix.data[j][i].abs() > augmented_matrix.data[pivot_row][i].abs() {
                pivot_row = j;
            }
        }

        if augmented_matrix.data[pivot_row][i].abs() < EPSILON {
            // 这里可以添加一些处理奇异矩阵的逻辑
            return vec![];
        }

        augmented_matrix.swap_rows(i, pivot_row);

        for j in i + 1..self.rows {
            let factor = augmented_matrix.data[j][i] / augmented_matrix.data[i][i];
            for k in i..self.cols + 1 {
                augmented_matrix.data[j][k] -= factor * augmented_matrix.data[i][k];
            }
        }
    }

    // 回代求解
    for i in (0..self.rows).rev() {
        let mut sum = 0.0;
        for j in i + 1..self.cols {
            sum += augmented_matrix.data[i][j] * solution[j - i - 1];
        }
        solution.push((augmented_matrix.data[i][self.cols] - sum) / augmented_matrix.data[i][i]);
    }

    // 归一化特征向量
    let norm = solution.iter().fold(0.0, |acc, &x| acc + x.powi(2)).sqrt();
    solution.iter().map(|&x| x / norm).collect()
}

// 辅助方法：交换矩阵的两行
fn swap_rows(&self, i: usize, j: usize) {
    let mut data_copy = self.data.clone();
    data_copy.swap(i, j);
    Matrix {
        rows: self.rows,
        cols: self.cols,
        data: data_copy,
    };
}
```



## 奇异值分解

奇异值分解（Singular Value Decomposition，简称SVD）是一种将一个矩阵分解成三个矩阵乘积的数学工具。对于任意的矩阵 $$A$$，其奇异值分解形式为：

$$ A = U \Sigma V^T $$

其中：

- $$U$$ 是一个列正交矩阵，其列向量称为左奇异向量。
- $$\Sigma$$ 是一个对角矩阵，其对角线上的元素称为奇异值。
- $$V^T$$ 是一个行正交矩阵的转置，其行向量称为右奇异向量。

### 解释和性质

1. **左奇异向量（Left Singular Vectors）：** $$U$$ 的列向量是 $$AA^T$$ 的特征向量。
2. **奇异值（Singular Values）：** $$\Sigma$$ 的对角线上的元素是 $$A$$ 的奇异值，是 $$A^TA$$ 或 $$AA^T$$ 特征值的平方根。
3. **右奇异向量（Right Singular Vectors）：** $$V$$ 的行向量是 $$A^T A$$ 的特征向量。





## 矩阵范数
