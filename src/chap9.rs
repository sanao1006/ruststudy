#![allow(unused_variables, dead_code)]
mod chap9 {
    fn magnitude(vector: &[f64; 3]) -> f64 {
        return vector
            .map(|v| v.powf(2.0))
            .iter()
            .fold(0.0, |sum, i| sum + i)
            .sqrt();
    }

    fn normalize(arr: &mut[f64; 3]) {
        let mag = magnitude(arr);
        for i in arr {
            *i = *i / mag;
        }
    }
}
