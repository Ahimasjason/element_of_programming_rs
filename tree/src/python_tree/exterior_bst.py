




def is_leaf(t) -> bool :
    return not t.left and not t.right

def exterior_tree(tree):
    
    def left_nodes(subtree, is_boundry):
        if not subtree:
            return []
        
        return (
            ([subtree] if  is_boundry or is_leaf(subtree) else []) + \
                left_nodes(subtree.left, True) + \
                left_nodes(subtree.right, is_boundry and not subtree.left)    
        )
    
    def right_nodes(subtree, is_boundry):
        if not subtree:
            return []
        
        return (
            ([subtree] if  is_boundry or is_leaf(subtree) else []) + 
                right_nodes(subtree.left, is_boundry and not subtree.right) +    
                right_nodes(subtree.right, True) 
        )
    
    # l_n = left_nodes(tree.left, True)
    # r_n = right_nodes(tree.right, True)
    # print("left tree ---- ", l_n)
    # print("right tree --- ",r_n)
    return ([tree] + left_nodes(tree.left, True) + right_nodes(tree.right, True) if tree else [])
    
    
    
# inorder_nulss = [
#     None,"D",None,"C","E",None,None
# ]

