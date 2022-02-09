


pub fn max_subarray(arr: Vec<isize>) -> Option<isize> {
    let mut chunks = Vec::new();
    for i in arr{
        if chunks.is_empty(){
            chunks.push(i);
        } else {
            let chunk : isize;

            if chunks[chunks.len() - 1] < 0 {
                chunk  = i;
            } else {
                chunk = chunks[chunks.len() -1 ] + i;
            }
            chunks.push(chunk);
        }
    }
    chunks.iter().max().map(|i| *i)
}

pub fn max_subarray_dp(arr: Vec<isize>) -> f64 {
    let mut previous_chunk = f64::NEG_INFINITY;
    let mut best_chunk = f64::NEG_INFINITY;
    for i in arr {
        let chunk : f64;
        if previous_chunk < 0f64 {
            chunk = i as f64;
        } else {
            chunk = previous_chunk + i as f64;
        }
        best_chunk = best_chunk.max(chunk );
        previous_chunk = chunk;
    }
    best_chunk
}


#[cfg(test)]
mod tests{
    use super::*;



    #[test]
    fn test_max_subarray(){
        let arr = vec![-1,1,2,3,-2];
        assert_eq!(max_subarray(arr), Some(6));
    }

    #[test]
    fn test_max_subarray_dp(){
        let arr = vec![-1,1,2,3,-2];
        assert_eq!(max_subarray_dp(arr), 6f64);
    }
}