

#[derive(Debug, PartialEq)]
pub struct MinMax{
    min : isize,
    max : isize,
}

impl MinMax {

    fn new(a: isize, b: isize) -> Self {
        
        if a < b {
            MinMax {min: a,max: b}
        } else {
            MinMax {min: b, max: a}
        }
    }
}



pub fn minimum_and_max(arr: Vec<isize>) -> MinMax {

    if arr.len() <= 1 {
        return MinMax::new(arr[0], arr[0]); 
    }

    let mut global  = MinMax::new(arr[0], arr[1]);

    for i in (2..(arr.len() - 1)).step_by(2) {

        let local = MinMax::new(arr[i], arr[i+ 1]);
        global = MinMax::new((global.min).min(local.min),
                             (global.max).max(local.max)
                            );

    }

    if arr.len() % 2 == 1 {

        global  = MinMax::new(
                                arr[arr.len() - 1].min(global.min), 
                                arr[arr.len() - 1].max(global.max)
                           )
    }
    global
}



#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_min_max(){
        let arr = vec![3,2,5,1,2,4];
        assert_eq!(minimum_and_max(arr), MinMax::new(1,5));
        let arr = vec![3,2,5,1,2,4,0];
        assert_eq!(minimum_and_max(arr), MinMax::new(0,5));
    }
}