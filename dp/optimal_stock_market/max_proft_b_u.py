def max_profit(daily_price):
    
    cash_not_owning_share = 0
    cash_owning_share = -float('inf')
    
    for price in daily_price:
        
        stragegy_buy = cash_not_owning_share - price
        stragegy_hold = cash_owning_share
        
        
        stragegy_sell = cash_owning_share + price
        stragegy_avoid = cash_not_owning_share
        
        
        cash_owning_share = max(stragegy_buy, stragegy_hold)
        cash_not_owning_share = max(stragegy_sell, stragegy_avoid)
        
    return cash_not_owning_share





if __name__ == "__main__":
    prices = [2,5,1]
    print(max_profit(prices))