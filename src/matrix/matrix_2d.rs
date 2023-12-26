use std::f64::EPSILON;

#[derive(Debug, PartialEq)]
pub struct Matrix {
    pub rows: usize,
    // 行数
    pub cols: usize,
    // 列数
    pub data: Vec<Vec<f64>>, // 存储矩阵元素的二维向量
}

impl Clone for Matrix {
    fn clone(&self) -> Self {
        let mut cloned_data = Vec::with_capacity(self.rows);

        for row in &self.data {
            let cloned_row: Vec<f64> = row.clone();
            cloned_data.push(cloned_row);
        }

        Matrix {
            rows: self.rows,
            cols: self.cols,
            data: cloned_data,
        }
    }
}
#[allow(dead_code)]
impl Matrix {
    // 创建一个新的矩阵
    pub fn new(data: Vec<Vec<f64>>) -> Matrix {
        // 检查输入数据的合法性
        let rows = data.len();
        assert!(rows > 0, "Matrix must have at least one row");

        let cols = data[0].len();
        assert!(cols > 0, "Matrix must have at least one column");

        for row in &data {
            assert_eq!(row.len(), cols, "All rows must have the same number of columns");
        }

        Matrix { rows, cols, data }
    }

    // 打印矩阵的方法
    pub fn print(&self) {
        for row in &self.data {
            for &elem in row {
                print!("{} ", elem);
            }
            println!();
        }
    }
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

    /// 矩阵的乘法
    pub fn multiply(&self, other: &Matrix) -> Matrix {
        // 检查矩阵维度是否允许相乘
        if self.cols != other.rows {
            panic!("Matrix dimensions do not match for multiplication.")
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

        Matrix {
            rows: self.rows,
            cols: other.cols,
            data: result_data,
        }
    }

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

                // 计算代数余子式
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
    // 计算矩阵的逆矩阵
    pub fn inverse(&self) -> Option<Matrix> {
        // 检查矩阵是否为方阵
        if self.rows != self.cols {
            return None;
        }

        // 递归计算行列式
        let det_a = self.determinant();
        if det_a.unwrap() == 0.0 {
            return None; // 行列式为零，逆矩阵不存在
        }

        // 计算矩阵的伴随矩阵
        let adj_a = self.adjoint();

        // 计算逆矩阵
        let inv_det = 1.0 / det_a.unwrap();
        let mut inv_a_data = Vec::with_capacity(self.rows);
        for i in 0..self.rows {
            let mut row = Vec::with_capacity(self.cols);
            for j in 0..self.cols {
                row.push(inv_det * adj_a.data[i][j]);
            }
            inv_a_data.push(row);
        }

        Some(Matrix {
            rows: self.rows,
            cols: self.cols,
            data: inv_a_data,
        })
    }
    // 计算矩阵的伴随矩阵
    fn adjoint(&self) -> Matrix {
        // 计算代数余子式
        let mut adj_a_data = Vec::with_capacity(self.rows);
        for i in 0..self.rows {
            let mut row = Vec::with_capacity(self.cols);
            for j in 0..self.cols {
                // 计算代数余子式
                let submatrix_data: Vec<Vec<f64>> = self.data
                    .iter()
                    .enumerate()
                    .filter(|&(row_idx, _)| row_idx != i)
                    .map(|(_, row)| row.iter().enumerate().filter(|&(col_idx, _)| col_idx != j).map(|(_, &val)| val).collect())
                    .collect();

                let submatrix = Matrix {
                    rows: self.rows - 1,
                    cols: self.cols - 1,
                    data: submatrix_data,
                };

                let cofactor = if (i + j) % 2 == 0 { 1.0 } else { -1.0 };
                row.push(cofactor * self.calculate_determinant(&submatrix.data).unwrap());
            }
            adj_a_data.push(row);
        }

        Matrix {
            rows: self.rows,
            cols: self.cols,
            data: adj_a_data,
        }
    }

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
    fn find_eigenvalues(&self, _det_a_minus_lambda_i: f64) -> Vec<f64> {
        let mut eigenvalues = Vec::with_capacity(self.rows);
        for i in 0..self.rows {
            eigenvalues.push(self.data[i][i]);
        }
        eigenvalues
    }

    // 辅助方法：求解特征向量
    fn solve_eigenvector(&self, eigenvalue: f64) -> Vec<f64> {
        let mut augmented_matrix_data = Vec::with_capacity(self.rows);
        for i in 0..self.rows {
            let mut row = Vec::with_capacity(self.cols + 1);
            for j in 0..self.cols {
                row.push(self.data[i][j] - eigenvalue * if i == j { 1.0 } else { 0.0 });
            }
            row.push(0.0); // 右侧的常数项
            augmented_matrix_data.push(row);
        }

        let mut augmented_matrix = Matrix {
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

    //////////////


    // 计算矩阵的奇异值分解
    pub fn svd(&self) -> (Matrix, Matrix, Matrix) {
        let mut u = self.clone();
        let mut vt = self.clone();

        let mut s = Matrix::zeros(self.cols, self.cols);

        // 初始化 U、Σ、V
        let mut ut = Matrix::eye(self.cols);
        let mut v = Matrix::eye(self.cols);

        // 迭代次数，可以根据需要调整
        let max_iter = 100;

        for _ in 0..max_iter {
            // U的计算
            let uu = u.transpose().multiply(&u);
            let (u_eigenvalues, u_eigenvectors) = uu.eigen();
            let u_sort_indices = Matrix::argsort(&u_eigenvalues);

            u = u.multiply_by_vector(&u_eigenvectors.column(u_sort_indices[0]));

            // V的计算
            let vv = vt.transpose().multiply(&vt);
            let (v_eigenvalues, v_eigenvectors) = vv.eigen();
            let v_sort_indices = Matrix::argsort(&v_eigenvalues);

            vt = vt.multiply_by_vector(&v_eigenvectors.column(v_sort_indices[0])).transpose();

            // 更新 Σ
            s.data = u.transpose().multiply(&self.multiply(&vt)).data;

            // 更新 U、V
            ut = ut.multiply(&u);
            v = v.multiply(&vt);
        }

        // Σ的对角线元素开平方得到奇异值
        for i in 0..self.cols {
            s.data[i][i] = s.data[i][i].sqrt();
        }

        (ut, s, v)
    }
    // 计算矩阵乘以向量
    pub fn multiply_by_vector(&self, vector: &Vec<f64>) -> Matrix {
        let mut result = Matrix::zeros(self.rows, 1);

        for i in 0..self.rows {
            for j in 0..1 {
                for k in 0..self.cols {
                    result.data[i][j] += self.data[i][k] * vector[k];
                }
            }
        }

        result
    }
    // 单位矩阵
    pub fn eye(size: usize) -> Matrix {
        let mut result = Matrix::zeros(size, size);
        for i in 0..size {
            result.data[i][i] = 1.0;
        }
        result
    }

    // 全0矩阵
    pub fn zeros(rows: usize, cols: usize) -> Matrix {
        Matrix {
            rows,
            cols,
            data: vec![vec![0.0; cols]; rows],
        }
    }
    // 求解特征值和特征向量
    pub fn eigen(&self) -> (Vec<f64>, Matrix) {
        // 简化处理，假设矩阵是实对称矩阵
        // 在实际应用中，可以使用更复杂的算法来处理一般情况
        let n = self.rows;
        let mut a = self.clone();
        let mut v = Matrix::eye(n);

        let mut d = Vec::with_capacity(n);
        for i in 0..n {
            d.push(a.data[i][i]);
        }

        // 迭代次数，可以根据需要调整
        let max_iter = 100;

        for _ in 0..max_iter {
            let mut sm = 0.0;
            for i in 0..n - 1 {
                for j in i + 1..n {
                    sm += f64::abs(a.data[i][j]);
                }
            }

            if sm == 0.0 {
                // 矩阵已经对称
                return (d, v);
            }

            let tresh = if a.data[0][0] < a.data[0][n - 1] {
                0.2 * sm / (n as f64)
            } else {
                0.2 * sm / (n as f64)
            };

            for ip in 0..n - 1 {
                for iq in ip + 1..n {
                    let g = 100.0 * f64::abs(a.data[ip][iq]);

                    if ip > 0 && ip < n - 1 {
                        let mut _h = 0.5 * (a.data[ip][ip] - a.data[iq][iq]);
                        let t = f64::sqrt(_h * _h + g * g);
                        _h = _h / t;
                        let c = f64::sqrt(0.5 + 0.5 * _h);
                        let  s = -0.5 * _h / c;
                        _h = 0.5 * (g / c) * (_h / c);
                        let mut b = Matrix::eye(n);
                        b.data[ip][ip] = c;
                        b.data[iq][iq] = c;
                        b.data[ip][iq] = s;
                        b.data[iq][ip] = -s;

                        let  at = b.transpose().multiply(&a).multiply(&b);
                        a = at.clone();
                        let  vt = b.multiply(&v);
                        v = vt.clone();
                    }

                    if f64::abs(a.data[ip][iq]) > tresh {
                        let _h = a.data[ip][ip] - a.data[iq][iq];
                        let t = if _h.abs() + g == _h.abs() {
                            a.data[ip][iq] / _h
                        } else {
                            let theta = 0.5 * _h / (a.data[ip][iq]);
                            1.0 / (f64::abs(theta) + f64::sqrt(1.0 + theta * theta))
                        };

                        let c = 1.0 / f64::sqrt(1.0 + t * t);
                        let s = t * c;

                        let _z = 1.0 / f64::sqrt(1.0 + t * t);
                        let mut b = Matrix::eye(n);
                        b.data[ip][ip] = c;
                        b.data[iq][iq] = c;
                        b.data[ip][iq] = s;
                        b.data[iq][ip] = -s;

                        let  at = b.transpose().multiply(&a).multiply(&b);
                        a = at.clone();
                        let  vt = b.multiply(&v);
                        v = vt.clone();
                    }
                }
            }
        }

        (d, v)
    }

    // 对向量进行排序并返回排序后的索引
    pub fn argsort(vector: &Vec<f64>) -> Vec<usize> {
        let mut indices: Vec<usize> = (0..vector.len()).collect();
        indices.sort_by(|&a, &b| vector[a].partial_cmp(&vector[b]).unwrap());
        indices
    }
    // 获取矩阵的某一列
    pub fn column(&self, col_index: usize) -> Vec<f64> {
        assert!(col_index < self.cols, "Column index out of bounds.");
        self.data.iter().map(|row| row[col_index]).collect()
    }

}
