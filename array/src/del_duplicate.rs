



fn rm_duplicate(arr : &mut Vec<usize>) -> usize {


    if arr.is_empty(){
        return 0
    }

    let mut write_idx = 1;

    for i in 1..arr.len() {

        if arr[write_idx - 1] != arr[i] {
            arr[write_idx] = arr[i];
            write_idx += 1
        }
    }
    write_idx
}






#[cfg(test)]
mod tests {

    use super::*;


    #[test]
    fn test_rm_dups(){
        let mut arr = vec![2,3,5,7,7,11,11,11,13];
        
        let idx = rm_duplicate(&mut arr);

        eprintln!("{:?}", &arr[..idx]);
    }

}