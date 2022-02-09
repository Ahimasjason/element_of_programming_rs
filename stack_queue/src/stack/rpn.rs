
use std::ops;




fn rpn(expr: Vec<char>) -> isize {

    let mut stack = Vec::new();


    for c in expr {

        match c {
            '+' |'-' | '*' | '/'  => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                let r = eval(c,a,b);
                stack.push(r);
            },
            _ if c.is_digit(10) => {
                stack.push(c.to_digit(10).unwrap() as isize);
            },
            _ => {}
        }
    }


    stack[stack.len() - 1]
}




fn eval(expr: char, a: isize,b: isize) -> isize {
    match expr {
                '*' => ops::Mul::mul(a,b),
                '+' => ops::Add::add(a,b),
                '-' => ops::Sub::sub(a,b),
                '/' => ops::Div::div(a,b),
                _ => 0
    }

}

fn rpn_pre(expr: Vec<char>) -> isize {


    let mut stack = Vec::new();
    //  0 should be operation
    let mut oper = expr[0];

    for i in 1..expr.len(){
        if expr[i].is_digit(10){
            stack.push(
                expr[i].to_digit(10).unwrap() as isize
            );
        } else {
            let a = stack.pop().unwrap();
            let b = stack.pop().unwrap();
            stack.push(eval(oper,a,b));
            oper = expr[i];

        }
    }
    
    eval(oper,stack.pop().unwrap(),stack.pop().unwrap())
}


#[test]
fn test_rpn(){

    let s = vec!['3','4','+','2','*','1','+'];

    assert_eq!(15, rpn(s));
    let s = vec!['3','4','+','2','*','1','+'];
    let s1 = vec!['+','3','4','*','2','+','1'];
    assert_eq!(rpn_pre(s1), rpn(s));
}