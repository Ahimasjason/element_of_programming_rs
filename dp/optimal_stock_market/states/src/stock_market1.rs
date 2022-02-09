fn max_profit1(prices: Vec<f64>) -> f64 {
    let mut cash_not_owning_share = 0f64;
    let mut cash_owning_share = f64::NEG_INFINITY;

    for price in prices {
        let buy = cash_not_owning_share - price;
        let hold = cash_owning_share;

        let sell = cash_owning_share + price;
        let avoid = cash_not_owning_share;

        cash_not_owning_share = sell.max(avoid);
        cash_owning_share = buy.max(hold);
    }
    cash_not_owning_share
}


fn max_profit_budget(prices: Vec<f64>, budget: f64) -> f64 {


    let mut cash_not_owning_share = budget;

    let mut cash_owning_share = f64::NEG_INFINITY;

    for price in prices{

        let buy = cash_not_owning_share - price;
        let hold = cash_owning_share;

        //  not owning state
        let sell = cash_owning_share + price;
        let avoid = cash_not_owning_share;


        cash_owning_share = buy.max(hold);
        if cash_owning_share < 0f64 {
            cash_owning_share = f64::NEG_INFINITY;

        }
        cash_not_owning_share = sell.max(avoid);
    }


    cash_not_owning_share - budget

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_max_profit_1() {
        let arr = vec![2f64, 5.0, 1.0, 3.0];
        assert_eq!(max_profit1(arr), 5.0);
    }

    #[test]
    fn test_max_profit_budget(){
        let arr = vec![2f64, 5.0, 1.0, 3.0];
        assert_eq!(max_profit_budget(arr, 2.0), 5.0);
    }
}
