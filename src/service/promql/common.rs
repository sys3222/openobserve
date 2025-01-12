/// Calculate mean over a slice of f64s
pub fn mean(data: &[f64]) -> Option<f64> {
    let sum = data.iter().sum::<f64>();
    let count = data.len();

    match count {
        positive if positive > 0 => Some(sum / count as f64),
        _ => None,
    }
}

/// Calculate std deviation over a slice of f64
pub fn std_deviation(data: &[f64]) -> Option<f64> {
    std_variance(data).map(|var| var.sqrt())
}

/// Calculate std deviation over a slice of f64
pub fn std_deviation2(data: &[f64], mean: f64, length: i64) -> Option<f64> {
    std_variance2(data, mean, length).map(|var| var.sqrt())
}

/// Calculate std variance over a slice of f64
pub fn std_variance(data: &[f64]) -> Option<f64> {
    match (mean(data), data.len()) {
        (Some(data_mean), count) if count > 0 => {
            let variance = data
                .iter()
                .map(|value| {
                    let diff = data_mean - *value;

                    diff * diff
                })
                .sum::<f64>()
                / count as f64;

            Some(variance)
        }
        _ => None,
    }
}

/// Calculate std variance over a slice of f64
pub fn std_variance2(data: &[f64], mean: f64, length: i64) -> Option<f64> {
    match (mean, length) {
        (data_mean, count) if count > 0 => {
            let variance = data
                .iter()
                .map(|value| {
                    let diff = data_mean - *value;

                    diff * diff
                })
                .sum::<f64>()
                / count as f64;

            Some(variance)
        }
        _ => None,
    }
}

pub fn quantile(data: &[f64], quantile: f64) -> Option<f64> {
    if quantile < 0 as f64 || quantile > 1_f64 || quantile.is_nan() {
        let value = match quantile.signum() as i32 {
            1 => f64::INFINITY,
            -1 => f64::NEG_INFINITY,
            _ => f64::NAN,
        };
        return Some(value);
    }
    if data.is_empty() {
        return None;
    }

    let mut sorted_data = data.to_vec();
    sorted_data.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let n = sorted_data.len();
    let index = (quantile * (n - 1) as f64) as usize;

    if index == n - 1 {
        return Some(sorted_data[index]);
    }

    let lower = sorted_data[index];
    let upper = sorted_data[index + 1];

    let fraction = quantile * (n - 1) as f64 - index as f64;
    let quantile_value = lower + (upper - lower) * fraction;

    Some(quantile_value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quantile() {
        let data = vec![4.0, 2.0, 1.0, 3.0, 5.0];
        let phi_quantile = 0.5;
        let result = quantile(&data, phi_quantile);
        let expected = 3.0;
        match result {
            Some(got) => assert_eq!(got, expected),
            None => assert!(false),
        }
    }
}
