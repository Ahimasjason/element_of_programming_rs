



fn snake_string(s: impl AsRef<str>) -> String {
    /*
    
    
    |________________________
    |_E______ -______ L______  0
    |H__L__ o__ W__ R__  D___  1
    |__   L______ O_______ !__ 2
     0 1 2 3 4 5 6 7 8  9 10
    */ 
    
    let string_vec = s.as_ref().chars().collect::<Vec<char>>();
    let mut result_vec = Vec::new();
    
    for i in (1..string_vec.len()).step_by(4){
        result_vec.push(string_vec[i]);
    }

    for k in (0..string_vec.len()).step_by(2){
        result_vec.push(string_vec[k]);
    }
    for j in (3..string_vec.len()).step_by(4){
            result_vec.push(string_vec[j]);
    }

    result_vec.into_iter().collect()
}




#[test]
fn test_snake_string(){
    println!("{:?}", snake_string("Hello_World!"));

    assert_eq!(snake_string("Hello_World!"), "e_lHloWrdlo!")
}   