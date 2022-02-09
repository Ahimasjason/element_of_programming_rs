from __future__ import annotations
from dataclasses import dataclass
from typing import Any
@dataclass
class Tree:
    data :  Any
    left: Tree = None
    right: Tree = None
    
    
    def insert(self, val : Any):
        
        
        if self.data > val:
            # move this to left
            if self.left is None:
                self.left = Tree(val)
            else:
                self.left.insert(val)
        else:
            if self.right is None:
                self.right = Tree(val)
            else:
                self.right.insert(val)
    
    
    def find_node(self, val : Any) -> Tree:
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
    