


from typing import ClassVar


def max_profit(daily_price, tx_limit):
    
    cash_not_owning_share = [-float('inf')] * (tx_limit + 1)
    cash_not_owning_share[0] = 0
    
    cash_owning_share = [-float('inf')] * (tx_limit + 1)
    
    for price in daily_price:
        cash_not_owning_share_next = [-float('inf')] * (tx_limit + 1)
        cash_owning_share_next = [-float('inf')] * (tx_limit + 1)
        
        for pr_tx_count in range(tx_limit):
            st_buy = cash_not_owning_share[pr_tx_count] - price
            st_hold = cash_owning_share[pr_tx_count]
            
            st_sell = cash_not_owning_share[pr_tx_count] + price
            st_avoid = cash_owning_share[pr_tx_count]
            
            if pr_tx_count < tx_limit:
                cash_not_owning_share_next[pr_tx_count + 1] = max(
                    cash_not_owning_share_next[pr_tx_count] + 1,
                    st_sell
                )
                
            cash_not_owning_share_next[pr_tx_count] = max (
                cash_not_owning_share_next[pr_tx_count],
                st_avoid
            )
            
            cash_owning_share_next[pr_tx_count] = max(
                cash_owning_share_next[pr_tx_count],
                st_buy,
                st_hold,
            )
        cash_not_owning_share = cash_not_owning_share_next
        cash_owning_share = cash_owning_share_next
    return max(cash_not_owning_share)



shares = [5,3,6,3,5,6,7,8,3,9,9]
TX_LIMIT = 3

print(max_profit(shares, TX_LIMIT))