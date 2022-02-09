



def duplicate(A):
    
    i = 0 
    
    l_p = len(A)
    while i < len(A) - 1:
        
        if A[i] == A[i + 1]:
            
            
            k = i + 1
            if i+ 2 >= len(A):
                break
            for j in range(i+2, len(A)):
                A[k] = A[j]
                k+= 1
            
        else:
            i+=1
    return i
             
             

def delete_dups(A):
    
    start_idx = 1
    
    for i in range(1, len(A)):
        
        if A[i] != A[i-1]:
            A[start_idx] = A[i]
            start_idx += 1
            
    return start_idx




def var1(A, elem):
    
    
    # last_idx = len(A) - 1
    # for i in range(0, len(A)):
        
    #     if A[i] == elem:
            
    #         if i <= last_idx: 
    #             k = i
    #             for j in range(i + 1, last_idx):
    #                 A[k] = A[j]
    #                 k += 1
    #                 last_idx -= 1
                    
                    
    start_idx = 0
    
    for i in range(len(A)):
        if A[i] != elem:
            A[start_idx] = A[i]
            start_idx+= 1                 
    return start_idx
        
# arr = [2,3,5,5,7,11,11,11,13]

# index = duplicate(arr)
# print(arr)
# print(index)
# print(arr[:index])        




arr = [2,3,5,5,7,11,11,11,13]

index = delete_dups(arr)
print(arr)
print(index)
print(arr[:index])



print("Var")
arr = [2,3,5,5,7,11,11,11,13]

index = var1(arr, 5)
print(arr)
print(index)
print(arr[:index])
