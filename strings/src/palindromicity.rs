


fn is_palindromecity(s: impl AsRef<str>) -> bool {

    let string = s.as_ref().to_lowercase().to_owned();
    let bytes = string.as_bytes();
    println!("{:?}", bytes);
    let (mut i, mut j) = (0, bytes.len() - 1);

    while i < j {

        while !bytes[i].is_ascii_alphanumeric() && i < j {
            i += 1;
        }
        while !bytes[j].is_ascii_alphanumeric() && i < j {
            j -= 1;
        }
        if bytes[i] != bytes[j]{
            return false
        }
        
        i += 1;
        j -= 1;

    }
    


    true
}


#[test]
fn test_palindromecity() {
    assert!(
        is_palindromecity("A man, a plan, a canal, Panama")
    );
    assert!(!is_palindromecity(&"Ray a Ray"));
}