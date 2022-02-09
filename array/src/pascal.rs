pub fn pascal_triangle(n: usize) -> Vec<Vec<usize>> {
    let mut pascal = (0..n).map(|i| vec![1; i + 1]).collect::<Vec<Vec<usize>>>();

    for i in 0..pascal.len() {
        for j in 1..i {
            let res = pascal[i - 1][j - 1] + pascal[i - 1][j];
            pascal[i][j] = res;
        }
    }
    pascal
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_pascal() {
        let n = 7;
        let pascal = pascal_triangle(7);
        // eprintln!("Pascal --. {:?}", pascal);
        for i in 0..n {
            for j in 1..i {
                assert_eq!(pascal[i][j], pascal[i - 1][j - 1] + pascal[i - 1][j]);
            }
        }
    }
}
