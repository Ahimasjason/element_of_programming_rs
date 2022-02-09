


// buy and sell stock once



pub fn once(arr: Vec<isize>) -> f64 
{

    let mut min_price = f64::INFINITY;
    let mut max_profit = 0f64;


    for price in arr {

        let today_profit = price as f64- min_price;
        max_profit = max_profit.max(today_profit);
        min_price = min_price.min(price as f64);
    }
    
    max_profit
}




#[cfg(test)]
mod tests{
    use super::*;


    #[test]
    fn test_once(){

        let arr = vec![310,315,275,295,260,270,290,230,255,250];
        assert_eq!(once(arr), 30f64);
    }
}