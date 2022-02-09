
use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    static ref  LETTER: HashMap<char, isize> = {
    let mut hash_map = HashMap::new();
    hash_map.insert('I',1);
    hash_map.insert('V',5);
    hash_map.insert('X',10);
    hash_map.insert('L',50);
    hash_map.insert('C',100);
    hash_map.insert('D',500);
    hash_map.insert('M',1000);

    hash_map
};

}

fn int_to_roman(num : usize) -> String {

    let m = ["", "M", "MM", "MMM"];
    let c = ["", "C", "CC", "CCC", "CD", "D",
         "DC", "DCC", "DCCC", "CM "];
    let x = ["", "X", "XX", "XXX", "XL", "L",
         "LX", "LXX", "LXXX", "XC"];
    let i = ["", "I", "II", "III", "IV", "V",
         "VI", "VII", "VIII", "IX"];

    let thousand = m[num / 1000];
    let hudered = c [(num % 1000) / 100];
    let tens = x[(num % 100) / 10];
    let ones = i[num % 10];

    let mut res = String::new();
    res.push_str(thousand);
    res.push_str(hudered);
    res.push_str(tens);
    res.push_str(ones);
    res
}


fn int_to_roman1(mut num : usize) -> String {

    let number = [1, 4, 5, 9, 10, 40, 50, 90,
                100, 400, 500, 900, 1000];
    let sym = ["I", "IV", "V", "IX", "X", "XL",
        "L", "XC", "C", "CD", "D", "CM", "M"];

    let mut res = String::new();
    for i in (0..sym.len()).rev(){

        while num >= number[i]{
            res.push_str(sym[i]);
            num = num - number[i];
        }
    }
    res
}

fn roman_to_int(s: String) -> isize {

    let  chars = s.chars().collect::<Vec<char>>();
    let mut i: isize = s.len() as isize - 1;
    let mut result = match LETTER.get(&chars[i as usize]){
        Some(val) => *val,
        None => 0 ,
    };
    if i <= 0 {
        return result
    }

    i -= 1;
    while i >= 0 {

        let iplace = LETTER.get(&chars[i as usize]).unwrap();
        let iplus1 = LETTER.get(&chars[i as usize +1]).unwrap();
        if iplace < iplus1 {
            result -= iplace;
        }else{
            result += iplace
        }
        i -= 1;
    }

    result
}


#[test]
fn test_roman_to_int(){

    assert_eq!(roman_to_int("I".into()), 1);
    assert_eq!(roman_to_int("II".into()), 2);
    assert_eq!(roman_to_int("III".into()), 3);
    assert_eq!(roman_to_int("IV".into()), 4);
    assert_eq!(roman_to_int("IX".into()), 9);
    assert_eq!(roman_to_int("XI".into()), 11);
    assert_eq!(int_to_roman(11), "XI");
    assert_eq!(int_to_roman(9), "IX");
    assert_eq!(int_to_roman(10), "X");
    assert_eq!(int_to_roman(11), "XI");
    assert_eq!(int_to_roman1(11), "XI");
    assert_eq!(int_to_roman1(9), "IX");
    assert_eq!(int_to_roman1(10), "X");
    
}