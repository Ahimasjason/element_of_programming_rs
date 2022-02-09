
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




def find_k_th(tree: Tree, elem: int):
    """Find the kth node appearing in an inorder traversal
        this does not includes the size variable
    Args:
        tree (Tree): [description]
        elem (int): [description]
    """
        
    s = []
    while tree or s:
        while tree:
            s.append(tree)
            tree = tree.left
        tree = s.pop()
        elem = elem - 1
        if elem == 0:
            break 
    
        tree = tree.right    
    return tree    
    
    
    
    
    