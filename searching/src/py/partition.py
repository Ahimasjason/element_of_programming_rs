import operator

def partition_around_pivot(arr, left, right, pivot_idx):
    
    new_pivot_idx = left
    pivot_val = arr[pivot_idx]
    arr[pivot_idx], arr[right] = arr[right], arr[pivot_idx]
    
    for i in range(left, right):
        if operator.gt(arr[i], pivot_val):
            arr[i], arr[new_pivot_idx] = arr[new_pivot_idx], arr[i]
            new_pivot_idx += 1
            
    arr[right], arr[new_pivot_idx] = arr[new_pivot_idx], arr[right]
    return new_pivot_idx




array = [3,2,1,5,6,4]
pivot = partition_around_pivot(
    array, 0, len(array) - 1, 0
)

print(array)
print(pivot)