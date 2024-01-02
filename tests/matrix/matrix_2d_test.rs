// 单元测试模块
#[cfg(test)]
mod tests {
    use rs_math::matrix::matrix_2d::Matrix2D;

    #[test]
    fn test_create_matrix() {
        // 测试创建一个 2x3 的矩阵
        let matrix_data = vec![
            vec![1.0, 2.0, 3.0],
            vec![4.0, 5.0, 6.0],
        ];

        let matrix = Matrix2D::new(matrix_data.clone());

        // 验证矩阵的行数和列数是否正确
        assert_eq!(matrix.rows, 2);
        assert_eq!(matrix.cols, 3);

        // 验证矩阵的数据是否正确
        assert_eq!(matrix.data, matrix_data);
    }
}