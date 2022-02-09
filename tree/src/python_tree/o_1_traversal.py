from __future__ import annotations
from tree import Tree
from parent_tree import ParentTree



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



def iterative_inorder(t: Tree):
    
    prev = None
    while t:
        if prev is t.parent:
            if t.left:
                n = t.left
            else:
                print(t.data)
                n = t.right or t.parent
        elif t.left is prev:
            print(t.data)
            n = t.right or t.parent
        else:
            n = t.parent
        
        prev, t = t, n
        
        
        
        
def iterative_preorder(t: Tree):
    prev = None
    
    while t:
        if prev is t.parent:
            print(t.data)
            if t.left:
                n = t.left
            else:
                n = t.right or t.parent
                
        elif prev is t.left:
            n = t.right or t.parent
        else:
            n = t.parent
        prev, t = t,n
            