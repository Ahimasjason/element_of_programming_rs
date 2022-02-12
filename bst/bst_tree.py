from __future__ import annotations
from dataclasses import dataclass
from typing import Any
@dataclass
class BST:
    data :  Any
    left: BST = None
    right: BST = None
    
    
    def insert(self, val : Any):
        
        def __insert(t: BST):
            
            if not t:
                return BST(data = val)
            
            if val < t.data:
                t.left = __insert(t.left)
            else:
                t.right = __insert(t.right)
            return t
        return __insert(self)
    
    def height(self, tree:BST) -> int:
        
        def __height(t):
            if not t:
                return 0
            lh = __height(t.left)
            rh = __height(t.right)
            if lh > rh:
                return lh + 1
            else:
                return rh + 1
        return __height(tree)
    
    
    def delete(self, val: Any):
        
        def __delete(t, val):
            if not t:
                return
            
            # if it is leaf then delete
            if not t.left and not t.right:
                del t
                return 
            
            if val < t.data:
                t.left = __delete(t.left, val)
            elif val > t.data:
                t.right = __delete(t.right, val)
            else:
                # now node to delete do not delete but trickle
                if self.height(t.left) > self.height(t.right):
                    pred = inorder_predecessor(t)
                    t.data = pred.data
                    t.left = __delete(t.left, pred.data)
                else:
                    succ = inorder_succssor(t)
                    t.data = succ.data
                    t.right = __delete(t.right, succ.data)
            return t
                    
        return __delete(self, val)
    
    
    def find_node(self, val : Any) -> BST:
        if self.data == val:
            return self
        elif self.data > val:
            return self.left.find_node(val) if self.left is not None else None
        else:
            return self.right.find_node(val) if self.right is not None else None
    
    def _printTree(self, node):
        if node is not None:
            self._printTree(node.left)
            print(str(node.data) + ' ')
            self._printTree(node.right)
    
    # def __repr__(self):
    #     return self._printTree(self)

def inorder_predecessor(t: BST):
    # left and right leaf node
    t = t.left
    while t and t.right:
        t = t.right
    return t

def inorder_succssor(t: BST):
    # left and right leaf node
    t = t.right
    while t and t.left:
        t = t.left
    return t
        
        
"""
bst = BST(40)
bst.insert(30)
bst.insert(15)  
bst.insert(10)
bst.insert(50) 
bst.insert(60)

"""