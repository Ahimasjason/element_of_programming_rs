from __future__ import annotations

from dataclasses import dataclass
from tree import Tree


@dataclass
class ParentTree(Tree):
    data :  Any
    parent: ParentTree = None
    left: ParentTree = None
    right: ParentTree = None
    
    
    # r = 10 v = 7
    def insert(self, val : Any):
        if self.data > val:
            #  insert on left 
            if not self.left:
                self.left = ParentTree(val, parent= self) 
            else:
                self.left.insert(val)
        else:
            if not self.right:
                self.right = ParentTree(val, parent= self)
            else:
                self.right.insert(val)
                
        
    