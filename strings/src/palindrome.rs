pub fn palindrome(string: impl AsRef<str>) -> bool {
    let st = string.as_ref().to_lowercase();
    let bytes = st.as_bytes();
    let mut mySize = 12;
    
    for i in 0..(bytes.len() / 2) {
        if bytes[i] != bytes[((bytes.len() - 1) - i) as usize] {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {

    use super::palindrome;

    #[test]
    fn test_palindrome() {
        assert!(!palindrome("Ahimas"));
        assert!(palindrome("Kayak"));
    }
}
