# 为了学好Rust也是拼了系列-数学库-统计

## 基本介绍

统计学是一门研究数据收集、分析、解释、呈现和组织的学科。它利用数学方法来理解和描述数据的特征，从而得出有关总体或样本的结论。以下是一些基本的统计学概念：

1. **数据：** 统计学关注收集的数据，这可以是数字、文字或图形，用于描述某个现象、事件或群体。

2. **描述统计：** 描述统计是对收集到的数据进行总结和展示的过程。这包括中心趋势度量（如平均数、中位数、众数）和离散趋势度量（如标准差、范围）等。

3. **推论统计：** 推论统计是通过对样本数据进行分析来推断总体特征的过程。这包括估计总体参数和测试关于总体的假设。

4. **概率：** 概率是事件发生的可能性，用数值来表示。在统计学中，概率经常用于描述随机现象，如投掷硬币或骰子的结果。

5. **统计变量和参数：** 统计变量是从样本中获得的数值，而参数是用于描述总体的数值。通过对样本数据进行分析，可以估计总体参数。

6. **假设检验：** 假设检验是通过对样本数据进行统计分析，评估对总体假设的支持程度。这涉及到设置假设、选择显著性水平和计算p值等步骤。

7. **回归分析：** 回归分析用于探讨变量之间的关系。简单线性回归涉及两个变量，而多元回归涉及多个变量。




## 平均值和方差

平均值（均值）和方差是统计学中常用的两个描述数据分布特征的统计量。

1. **平均值：**
   平均值是一组数据的总和除以数据的个数。对于给定的数据集 {x₁, x₂, ..., xn}，其平均值（μ）的数学表示为：

$$ \mu = \frac{1}{n} \sum_{i=1}^{n} x_i $$

其中，$$\sum_{i=1}^{n} x_i$$ 表示数据集中所有元素的和，n 是数据集中元素的个数。

2. **方差：**
   方差衡量了数据集中各个数据点与平均值之间的差异程度。方差越大，表示数据点越分散；方差越小，表示数据点越集中。对于给定的数据集 {x₁, x₂, ..., xn}，其方差（σ²）的数学表示为：

$$ \sigma^2 = \frac{1}{n} \sum_{i=1}^{n} (x_i - \mu)^2 $$

其中，$$\mu$$ 是数据集的平均值，$$(x_i - \mu)$$ 表示每个数据点与平均值的差异，平方后求和，最后除以数据点的个数 n。

方差的平方根被称为标准差，用来度量数据的离散程度，数学表示为：

$$ \sigma = \sqrt{\sigma^2} $$



**程序解如下**

```
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


```





## 中位数和众数

中位数和众数是描述数据集中位置和频率的两个统计量。

1. **中位数：**
中位数是一组数据按大小排列后位于中间位置的值。对于有奇数个数据点的数据集，中位数就是中间那个数；对于有偶数个数据点的数据集，中位数是中间两个数的平均值。数学上，如果有序数据集为 {x₁, x₂, ..., xn}，则中位数 $$M$$ 的计算方式为：

   - 如果 $$n$$ 为奇数：$$ M = x_{\frac{n+1}{2}} $$
   - 如果 $$n$$ 为偶数：$$ M = \frac{x_{\frac{n}{2}} + x_{\frac{n}{2}+1}}{2} $$

2. **众数：**
众数是数据集中出现频率最高的值。一个数据集可能有一个众数、多个众数（称为多峰分布），也可能没有众数。数学上，如果数据集 {x₁, x₂, ..., xn} 中某个数值 $$a$$ 出现的次数最多，那么 $$a$$ 就是数据集的众数。

这两个统计量在描述数据集的中心趋势和分布形状上起到不同的作用。中位数对于受极端值影响较大的数据集来说比平均值更具鲁棒性。众数则用于表示数据中的集中趋势，特别是在处理离散型数据时。



**程序解如下**

```
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
        let count = frequency_map.entry(value).or_insert(0);
        *count += 1;
    }

    // 找到出现次数最多的频率
    let max_frequency = frequency_map.values().cloned().max().unwrap_or(0);

    // 找到所有出现次数等于最大频率的元素
    let modes: Vec<f64> = frequency_map
        .iter()
        .filter(|&(_, &frequency)| frequency == max_frequency)
        .map(|(&value, _)| value)
        .collect();

    modes
}
```



## 标准差

标准差和方差是描述数据分散程度的两个统计量。它们都是用来衡量数据集中各个数据点与数据集平均值的偏离程度。

1.   **标准差：**
     标准差是方差的平方根。标准差的计算方式为：

$$ \sigma = \sqrt{\sigma^2} $$

标准差与方差的区别在于，标准差的量纲与原始数据一致，而方差的量纲是原始数据的平方。

标准差和方差的值越大，说明数据点与平均值的偏离程度越大，数据集的分散程度越大；反之，值越小，说明数据点越集中，分散程度越小。这两个统计量在统计学和数据分析中被广泛用于衡量数据的变异性和离散程度。

**程序解如下**

```
// 计算标准差的函数
pub fn calculate_standard_deviation(data: &[f64]) -> f64 {
    let variance = calculate_variance(data);
    variance.sqrt()
}

```





## 连续性检验




### Shapiro-Wilk



Shapiro-Wilk 检验的统计量是基于观测数据的排序值和正态分布的期望值之间的协方差。设我们有一个样本数据集 $$x_1, x_2, ..., x_n$$，首先对数据进行排序得到 $$x_{(1)}, x_{(2)}, ..., x_{(n)}$$，其中 $$x_{(1)}$$ 是最小值，$$x_{(n)}$$ 是最大值。

Shapiro-Wilk 统计量的计算如下：

$$ W = \frac{\left(\sum_{i=1}^{n} a_i x_{(i)}\right)^2}{\sum_{i=1}^{n} (x_i - \bar{x})^2} $$

其中，$$a_i$$ 是与样本大小 $$n$$ 相关的一组常数。如果数据集符合正态分布，统计量 $$W$$ 应该接近于 1。如果数据不符合正态分布，$$W$$ 将较小。

最后，通过与预先计算的分布的理论值相比较，可以计算出一个 p-value，用于判断是否拒绝零假设，即数据符合正态分布。

程序解如下

```
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
```







### **Kolmogorov-Smirnov（KS）检验：**

Kolmogorov-Smirnov 检验是一种用于检验一个样本是否符合特定分布的非参数统计方法。最常见的应用是检验一个样本是否符合正态分布，但它也可以用于检验其他理论分布，如指数分布、均匀分布等。

**基本思想：**

KS 检验的基本思想是通过比较样本累积分布函数（ECDF）与理论分布的累积分布函数（CDF）之间的最大差异来判断样本是否来自于某个理论分布。

**KS 统计量的计算：**

给定一个样本 $$X = \{x_1, x_2, ..., x_n\}$$ 和一个理论分布的累积分布函数 $$F(x)$$，KS 统计量（D）的计算步骤如下：

1. 将样本数据排序，得到 $$X_{\text{sorted}} = \{x_{(1)}, x_{(2)}, ..., x_{(n)}\}$$。
2. 计算每个观测值的 ECDF 和理论分布的 CDF 在排序后的数据点处的差异：$$D_i = |F(x_{(i)}) - \frac{i}{n}|$$
3. 选择最大的差异作为 KS 统计量：$$D = \max_i D_i$$

**优点和注意事项：**

- **非参数性：** KS 检验是一种非参数方法，不依赖于样本数据的分布。
- **对称性：** KS 检验对于连续分布的检验相对于其他方法具有优势，但对于离散分布的检验效果可能较差。
- **样本大小：** 对于小样本，KS 检验可能不够敏感。



**扩展**

Cumulative Distribution Function (CDF) 是一个表示累积概率的函数，对于连续型随机变量 X，它定义为：

$$ F(x) = P(X \leq x) $$

对于离散型随机变量，CDF 的定义是：

$$ F(x) = P(X \leq x) = \sum_{i \leq x} P(X = i) $$

在 Kolmogorov-Smirnov 检验中，需要根据你所关心的理论分布替换为相应的 CDF 计算方法。以下是一个简单的正态分布 CDF 的实现：

```rust

fn erf(x: f64) -> f64 {
    const a1: f64 =  0.254829592;
    const a2: f64 = -0.284496736;
    const a3: f64 =  1.421413741;
    const a4: f64 = -1.453152027;
    const a5: f64 =  1.061405429;
    const p: f64 =  0.3275911;

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
```

在这个例子中，使用了正态分布的 CDF 计算。请注意，这只是一个示例，具体应用中你可能需要根据你的数据和假设使用不同的分布和相应的 CDF 函数。



### **Anderson-Darling 检验：**

Anderson-Darling 检验是一种用于检验一个样本是否来自于特定分布的统计检验方法，通常用于检验样本是否来自于正态分布。它对尾部的敏感性更高，因此在小样本和对尾部分布特征敏感的情况下较为适用。

**基本思想：**

Anderson-Darling 检验的基本思想是通过比较样本的累积分布函数（ECDF）与理论分布的累积分布函数（CDF）之间的差异来判断样本是否来自于某个理论分布。

**统计量的计算：**

给定一个样本 $$X = \{x_1, x_2, ..., x_n\}$$ 和一个理论分布的累积分布函数 $$F(x)$$，Anderson-Darling 统计量（A²）的计算步骤如下：

1. 将样本数据排序，得到 $$X_{\text{sorted}} = \{x_{(1)}, x_{(2)}, ..., x_{(n)}\}$$。
2. 计算每个观测值的 ECDF 和理论分布的 CDF 在排序后的数据点处的差异：$$A^2 = -n - \frac{1}{n}\sum_{i=1}^{n} (2i-1) \left[\ln F(x_{(i)}) + \ln(1 - F(x_{(n-i+1)}))\right]$$
3. 将统计量 $$A^2$$ 与临界值进行比较，以进行假设检验。

**优点和注意事项：**

- **对尾部的敏感性：** Anderson-Darling 检验对尾部分布特征的敏感性较高，适用于对极端值敏感的情况。
- **样本大小：** 对于小样本，Anderson-Darling 检验可能更为适用。
- **适用范围：** 适用于多种理论分布的检验，不仅限于正态分布。



**程序解如下**

```
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
```

### 指数分布

指数分布是概率论和统计学中一种重要的连续概率分布，通常用于建模独立随机事件发生的时间间隔。指数分布具有无记忆性的特性，这意味着给定随机变量 X 的值，未来发生事件的等待时间与过去的等待时间无关。

**指数分布的概率密度函数（PDF）:**

指数分布的概率密度函数如下：

$$
f(x;\lambda) = 
\begin{cases} 
\lambda e^{-\lambda x} & \text{if } x \geq 0 \\
0 & \text{if } x < 0 
\end{cases}
$$

其中，$$ \lambda > 0 $$ 是分布的一个参数，通常称为速率参数。指数分布的概率密度函数表示了在给定时间间隔内事件发生的概率。

**累积分布函数（CDF）:**

指数分布的累积分布函数是概率密度函数的积分：

$$
F(x;\lambda) = 
\begin{cases} 
1 - e^{-\lambda x} & \text{if } x \geq 0 \\
0 & \text{if } x < 0 
\end{cases}
$$

**期望值和方差:**

指数分布的期望值（均值）为 $$ \frac{1}{\lambda} $$，方差为 $$ \frac{1}{\lambda^2} $$。这意味着随机事件发生的平均时间间隔为 $$ \frac{1}{\lambda} $$，方差越小，事件发生的时间间隔越稳定。

**解释:**

- **无记忆性：** 指数分布的重要特性是无记忆性，即在任何时刻 t，给定 $$X > t$$，未来的等待时间与过去的等待时间无关。这使得指数分布在建模许多随机事件的时间间隔时非常有用。

- **事件发生率：** 速率参数 $$ \lambda $$ 表示事件在单位时间内发生的平均次数。较大的 $$ \lambda $$ 表示事件更频繁发生，较小的 $$ \lambda $$ 表示事件发生较少。

- **期望值和方差：** 期望值和方差提供了关于事件发生时间分布的中心位置和离散程度的信息。期望值越大，事件发生的平均时间间隔越长；方差越小，事件发生的时间间隔越稳定。









### 对数正态分 
### P-P图 Probability-Probability Plot
### Q-Q图 Quantile-Quantile Plot


## 离散性检验