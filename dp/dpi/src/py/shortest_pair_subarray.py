


def find_shortest_subarray_sum(arr, target):
    
    result = float('inf')
    window_sum = 0
    left = 0 
    min_length_upto = [float('inf')] * len(arr)
    
    for r, val in enumerate(arr):
        
        min_length_upto[r] = min_length_upto[r-1]
        window_sum += val
        
        while window_sum > target:
            window_sum -= arr[left]
            left -= 1
            
        if window_sum == target:
            length = r - left + 1
            result = min(
                result,
                min_length_upto[left - 1] + length
            )
            min_length_upto[r] = min(min_length_upto[r], length)
            
    
    if result == float('inf'):return 0
    return result



arr = [1,2,1,1,1]
t_sum = 3
find_shortest_subarray_sum(arr, t_sum)