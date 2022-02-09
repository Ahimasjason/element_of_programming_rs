
def can_reach_end(A):
    
    further_reach_so_far, last_idx = 0, len(A) - 1
    
    i = 0
    st = []
    while i <= further_reach_so_far and further_reach_so_far < last_idx :
        curr_val = A[i] + i
        st.append(further_reach_so_far)
        further_reach_so_far = max(further_reach_so_far, curr_val)
        i+=1
    return further_reach_so_far >= last_idx 


# arr = [3,2,0,0,2,0,1]
arr = [0,2,0,1]
res = can_reach_end(arr)
print(res)