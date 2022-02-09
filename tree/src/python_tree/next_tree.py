from __future__ import annotations

from typing import Any
from dataclasses import dataclass
from tree import Tree

@dataclass
class NextTree(Tree):
    data :  Any
    left: NextTree = None
    right: NextTree = None
    next: NextTree = None



preorder = "A B C D E F G H I J K L M N O".split(" ")
inorder =  "D C E B G F H A K J L I N M O".split(" ")


def build_next_tree(inorder, preorder):
    hm = {data:i for i, data in enumerate(inorder)}
    
    
    def __build(preorder_start, preorder_end, inorder_start, inorder_end):
        
        
        if preorder_end <= preorder_start or inorder_end <= inorder_start:
            return None
        
        node_idx = hm[preorder[preorder_start]]
        left_size = node_idx - inorder_start
        
        return NextTree(
            data = preorder[preorder_start],
            left = __build(
                    preorder_start + 1 , preorder_start + 1 + left_size, inorder_start,node_idx
                ),
            right = __build(
                    preorder_start + 1 + left_size, preorder_end, node_idx + 1, inorder_end
                ),
            next = None
        )
        
        
    
    
    return __build(0, len(preorder), 0, len(inorder))


t = build_next_tree(inorder, preorder)




def compute_right_tree(tree):
    while tree and tree.left:
        t = tree
        while t and t.left:
            t.left.next = t.right
            t.right.next = t.next and t.next.left
            t = t.next
        tree = tree.left
        
        
