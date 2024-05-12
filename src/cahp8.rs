#![allow(unused_variables, dead_code)]
#[cfg(test)]
mod chap8test {
    fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
        let mut array: [[i32; 3]; 3] = [[0; 3]; 3];
        for i in 0..3 {
            for j in 0..3 {
                array[i][j] = matrix[j][i]
            }
        }
        return array;
    }

    #[test]
    fn test_transpose() {
        let matrix = [
            [101, 102, 103], //
            [201, 202, 203],
            [301, 302, 303],
        ];
        let transposed = transpose(matrix);

        assert_eq!(
            transposed,
            [
                [101, 201, 301], //
                [102, 202, 302],
                [103, 203, 303],
            ]
        );
    }
}
