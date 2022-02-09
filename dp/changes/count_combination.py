









def count_way_to_pay(amount, coins):
    
    
    def helper(index, amount):
        
        if amount == 0:
            return 1
        if amount <0 or index == len(coins):
            return 0
        pay_count = 0
        coin = coins[index]
        for repeats in range(amount // coin + 1):
            payment = coin * repeats
            pay_count += helper(index + 1, amount - payment)
        return pay_count
    
    return helper(0, amount)



coins = [1,2,5]
amount = 6
count_way_to_pay(amount, coins)