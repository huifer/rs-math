use std::collections::HashMap;

pub struct Statistics {}

// 计算平均值的函数
pub fn calculate_mean(data: &[f64]) -> f64 {
    let sum: f64 = data.iter().sum();
    let count = data.len() as f64;

    sum / count
}

// 计算方差的函数
pub fn calculate_variance(data: &[f64]) -> f64 {
    let mean = calculate_mean(data);
    let squared_diff: f64 = data.iter().map(|x| (x - mean).powi(2)).sum();
    let count = data.len() as f64;

    squared_diff / count
}

// 计算中位数的函数
pub fn calculate_median(data: &mut Vec<f64>) -> f64 {
    data.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = data.len();

    if n % 2 == 0 {
        // 对于偶数个数据点，取中间两个数的平均值
        let mid1 = data[(n / 2) - 1];
        let mid2 = data[n / 2];
        (mid1 + mid2) / 2.0
    } else {
        // 对于奇数个数据点，取中间那个数
        data[n / 2]
    }
}


// 计算众数的函数
pub fn calculate_mode(data: &[f64]) -> Vec<f64> {
    let mut frequency_map = HashMap::new();

    // 统计每个元素出现的频率
    for &value in data {
        *frequency_map.entry(value.to_bits()).or_insert(0) += 1;
    }

    // 找到最大的频率
    let max_frequency = frequency_map.values().cloned().max().unwrap_or_default();

    // 找到所有出现次数等于最大频率的元素
    let modes: Vec<f64> = frequency_map
        .into_iter()
        .filter(|(_, frequency)| *frequency == max_frequency)
        .map(|(value_bits, _)| f64::from_bits(value_bits))
        .collect();

    modes
}

// 计算标准差的函数
pub fn calculate_standard_deviation(data: &[f64]) -> f64 {
    let variance = calculate_variance(data);
    variance.sqrt()
}



// 返回元组，包含 Shapiro-Wilk 统计量和 p-value
pub fn shapiro_wilk_test(data: &[f64]) -> (f64, f64) {
    let n = data.len();
    let mut sorted_data = data.to_vec();
    sorted_data.sort_by(|a, b| a.partial_cmp(b).unwrap());

    // 计算 W 统计量
    let a: Vec<f64> = (1..=n).map(|i| {
        (2.0 * i as f64 - 1.0) / n as f64
    }).collect();
    let w_numer = a.iter().zip(&sorted_data).map(|(ai, xi)| ai * xi).sum::<f64>();
    let w_denom = data.iter().map(|xi| (xi - mean(&data)).powi(2)).sum::<f64>();
    let w = (w_numer.powi(2) / w_denom).round();

    // 计算 p-value
    let mu = 0.316 * n as f64 + 0.183;
    let sigma = 0.001 * n as f64 + 0.1;
    let z = (w - mu) / sigma;

    let p_value = 2.0 * (1.0 - z.abs().exp());

    (w, p_value)
}

pub fn mean(data: &[f64]) -> f64 {
    data.iter().sum::<f64>() / data.len() as f64
}
fn erf(x: f64) -> f64 {
    let a1: f64 =  0.254829592;
    let a2: f64 = -0.284496736;
    let a3: f64 =  1.421413741;
    let a4: f64 = -1.453152027;
    let a5: f64 =  1.061405429;
    let p: f64 =  0.3275911;

    let sign = if x < 0.0 { -1.0 } else { 1.0 };
    let x = x.abs();

    let t = 1.0 / (1.0 + p * x);
    let y = (((((a5 * t + a4) * t) + a3) * t + a2) * t) + a1;

    sign * (1.0 - y * (-x * x).exp())
}



// 计算正态分布的累积分布函数（CDF）
fn normal_cdf(x: f64, mean: f64, stddev: f64) -> f64 {
    0.5 * (1.0 + erf((x - mean) / (stddev * (2.0_f64.sqrt())) / 2.0))
}

// 计算 Kolmogorov-Smirnov 统计量
fn kolmogorov_smirnov_test(data: &[f64]) -> f64 {
    let n = data.len();
    let mut sorted_data = data.to_vec();
    sorted_data.sort_by(|a, b| a.partial_cmp(b).unwrap());

    // 计算最大差异
    let d = (1..=n).map(|i| {
        let observed_cdf = i as f64 / n as f64;
        let expected_cdf = normal_cdf(sorted_data[i - 1], mean(&data), stddev(&data));
        (observed_cdf - expected_cdf).abs()
    }).fold(0.0, f64::max);

    d
}


// 计算样本数据的标准差
fn stddev(data: &[f64]) -> f64 {
    let m = mean(data);
    (data.iter().map(|x| (x - m).powi(2)).sum::<f64>() / data.len() as f64).sqrt()
}

// 计算 Anderson-Darling 统计量
pub fn anderson_darling_test(data: &[f64]) -> f64 {
    let n = data.len();
    let mut sorted_data = data.to_vec();
    sorted_data.sort_by(|a, b| a.partial_cmp(b).unwrap());

    // 计算统计量 A^2
    let a_squared = -(n as f64) - (1.0 / n as f64) * (1..=n).fold(0.0, |acc, i| {
        let term = (2.0 * i as f64 - 1.0) * (normal_cdf(sorted_data[i - 1], mean(&data), stddev(&data)).ln() +
            (1.0 - normal_cdf(sorted_data[n - i], mean(&data), stddev(&data))).ln());
        acc + term
    });

    a_squared
}

fn fit_exponential(data: &[f64]) -> f64 {
    // 使用最小二乘法拟合指数分布
    let lambda = 1.0 / mean(data);

    lambda
}
// 计算 Anderson-Darling 统计量
fn anderson_darling_test_2(data: &[f64], lambda: f64) -> f64 {
    let n = data.len();
    let mut sorted_data = data.to_vec();
    sorted_data.sort_by(|a, b| a.partial_cmp(b).unwrap());

    // 计算统计量 A^2
    let a_squared = -(n as f64) - (1.0 / n as f64) * (1..=n).fold(0.0, |acc, i| {
        let term = (2.0 * i as f64 - 1.0) * (exponential_pdf(sorted_data[i - 1], lambda).ln() +
            (1.0 - exponential_pdf(sorted_data[n - i], lambda)).ln());
        acc + term
    });

    a_squared
}
// 计算指数分布的概率密度函数（PDF）
fn exponential_pdf(x: f64, lambda: f64) -> f64 {
    if x < 0.0 {
        return 0.0;
    }
    lambda * (-lambda * x).exp()
}

// 判断数据是否符合指数分布
fn is_exponential(data: &[f64], _significance_level: f64) -> bool {
    let lambda = fit_exponential(data);
    let ad_statistic = anderson_darling_test_2(data, lambda);

    // 根据显著性水平和临界值判断是否拒绝原假设
    let critical_value = 1.0; // 这里需要根据显著性水平和样本大小选择适当的临界值
    ad_statistic <= critical_value
}