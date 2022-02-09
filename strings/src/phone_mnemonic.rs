
const PHONE_MNEMONIC : [&'static str; 10] = ["0","1","ABC","DEF","GHI","JKL","MNO","PQRS","TUV","WXYZ"];




/*
def letterCombinationsUtil(number, n, table):
 
    list = []
    q = deque()
    q.append("")
 
    while len(q) != 0:
        s = q.pop()
 
        // If complete word is generated
        // push it in the list
        if len(s) == n:
            list.append(s)
        else:
 
            // Try all possible letters for current digit
            // in number[]
            for letter in table[number[len(s)]]:
                q.append(s + letter)
 
    # Return the generated list
    return list
 */ 



fn mnemonic(digit : impl AsRef<str>) -> Vec<String> {

    let mut string : Vec<char> = digit.as_ref().chars().collect();
    
    let mut res = Vec::new();
    let mut stack = Vec::new();
    stack.push("".to_string());
    while !stack.is_empty(){
        let c = stack.pop().unwrap();
        if c.len() == digit.as_ref().len(){
            res.push(c);
        }else {
            let idx = string[c.len()].to_digit(10).unwrap()  as usize;

            for i in PHONE_MNEMONIC[idx].chars(){
                stack.push(c.to_owned() + &i.to_string());
            }
        }
    }
    let len = res.len();
    for i in 0..len / 2 {
        res.swap(i, len - i - 1);
    }
    res
}


#[test]
fn test_mnemonic(){

    let res = mnemonic("23");
    // ["ad","ae","af","bd","be","bf","cd","ce","cf"]
    println!("mnemonic -> {:?}", res);
    assert_eq!(res,["AD","AE","AF","BD","BE","BF","CD","CE","CF"]);
}