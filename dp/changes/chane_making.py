
def make_solution(coins, amount):
    solutions = [None] * (amount +1)
    solutions[0] = []
    paid = 0
    while paid < amount:
        if solutions[paid] is not None:
            for coin in coins:
                next_paid = coin + paid
                if next_paid > amount:
                    continue
                if (solutions[next_paid] is None or 
                    len(solutions[next_paid]) > len(solutions[paid]) + 1):
                    
                    solutions[next_paid] = solutions[paid] + [coin]
                    
        paid += 1
    return solutions[amount]
    

coins = [5,2,1]
amount = 9
make_solution(coins, amount)