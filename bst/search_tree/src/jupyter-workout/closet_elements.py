
import bintrees


s_arrays = ([5,10,15],[3,6,9,12,15], [8,16,24])

def find_closest(sorted_arrays):
    
    min_dist_so_far = float('nan')
    iters = bintrees.RBTree()
    
    for idx, sorted_array in enumerate(sorted_arrays):
        it = iter(sorted_array)
        first_min = next(it, None)
        if first_min:
            iters.insert((first_min, idx), it)
            
    
    while True:
        min_val, min_idx = iters.min_key()
        max_val = iters.max_key()[0] 
        min_dist_so_far = min(max_val - min_val, min_dist_so_far)
        it = iters.pop_min()[1]
        next_min = next(it, None)
        if not next_min:
            return min_dist_so_far
        iters.insert((next_min, min_idx), it)