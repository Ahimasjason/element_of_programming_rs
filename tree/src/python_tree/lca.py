
from typing import Tuple
from tree import Tree
from parent_tree import ParentTree


root = Tree(7)
root.insert(4)
root.insert(8)
root.insert(5)
root.insert(10)
root.insert(6)
root.insert(9)
root.insert(11)
root.insert(3)
root.insert(5)


proot = ParentTree(7)
proot.insert(4)
proot.insert(8)
proot.insert(5)
proot.insert(10)
proot.insert(6)
proot.insert(9)
proot.insert(11)
proot.insert(3)
proot.insert(5)



def find_lca(node: Tree, node_0: Tree, node_1:Tree):
    
    def __find(node) -> Tuple:
        # print(node)
        if not node:
            return (0, None)
        
        left_res = __find(node.left)
        if left_res[0] == 2:
            return left_res
        
        right_res = __find(node.right)
        if right_res[0] == 2:
            return right_res
        
        num_cal = (
            left_res[0] + right_res[0] + int(node is node_0) + int(node is node_1) 
        )
        # print(num_cal)
        return (num_cal , node if num_cal == 2 else None)
    res = __find(node)
    # print(res)
    return res[1]



def find_lca_p(r: ParentTree, node_0: ParentTree, node_1:ParentTree) -> ParentTree:
    
    def find_depth(node):
        depth = 0
        # root node wont have depth
        while node:
            depth += 1
            node = node.parent
        return depth
    
    depth_0, depth_1 = find_depth(node_0), find_depth(node_1)
    
    if depth_1 > depth_0:
        node_0 , node_1 = node_1, node_0
        
    diff = abs(depth_0 - depth_1)
    while diff:
        node_0 = node_0.parent
        diff -= 1
        
    while node_0 is not node_1:
        node_0, node_1 = node_0.parent, node_1.parent
    return node_0



def left_view(t: Tree):
    
    def __left(n):
        if n:
            print(n.data)
            __left(n.left)
            __left(n.right)
    
    __left(t.left)
    
    

    
            

l = root.find_node(9)
r = root.find_node(11)