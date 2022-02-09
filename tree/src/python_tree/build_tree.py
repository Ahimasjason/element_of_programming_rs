from tree import Tree





def build_binary_tree(preorder, inorder):
    
    hm = {data: i for i, data in enumerate(inorder)}
    
    def _build(preorder_start, preorder_end, inorder_start, inorder_end):
        
        if preorder_end <= preorder_start or inorder_end <= inorder_start:
            return None
        
        node_root_idx = hm[preorder[preorder_start]]
        left_node_size = node_root_idx - inorder_start
        
        return Tree(
            data=preorder[preorder_start],
            left = _build(
                preorder_start + 1, preorder_start + 1 + left_node_size,
                inorder_start , node_root_idx
            ),
            right = _build(
               preorder_start + 1+ left_node_size, preorder_end,
               node_root_idx + 1, inorder_end 
            )
        )
    return _build(
        0, len(preorder), 0 , len(inorder)
    )
        


def build_post_binary_tree(postorder, inorder):
    
    hm = {data: i for i, data in enumerate(inorder)}
    
    def _build(postorder_start, postorder_end, inorder_start, inorder_end):
        
        if postorder_end <= postorder_start or inorder_end <= inorder_start:
            return None
        
        node_root_idx = hm[postorder[postorder_end]]
        left_node_size = node_root_idx - inorder_start
        
        return Tree(
            data=postorder[postorder_end],
            left = _build(
                postorder_start ,  left_node_size,
                inorder_start , node_root_idx
            ),
            right = _build(
               postorder_start + 1+ left_node_size, postorder_end,
               node_root_idx + 1, inorder_end 
            )
        )
    return _build(
        0, len(postorder) - 1, 0 , len(inorder) - 1
    )
    
inoder_seq = "F B A E H C D I G".split(" ")
preorder_seq = "H B F E A C D G I".split(" ")
postorder_seq = "F A E B I G D C H".split(" ")



class reverse_iterator:
    def __init__(self, collection):
        self.data = collection
        self.index = len(self.data)
    def __iter__(self):
        return self
    def __next__(self):
        if self.index == 0:
            raise StopIteration
        self.index = self.index - 1
        return self.data[self.index]

def reconstruct_preorder(preorder):
    
    def __build(iterator):
        key = next(iterator)
        if not key:
            return None
        left_tree = __build(iterator)
        right_tree = __build(iterator)
        return Tree(key,left_tree, right_tree)
    
    return __build(iter(preorder))

def reconstruct_postorder(postorder):
    
    def __build(idx):
        # if not idx:
        #     return None
        key = next(idx)
        if not key:
            return None
        # idx = idx-1
        right_tree = __build(idx)
        left_tree = __build(idx)
        
        return Tree(key,left_tree, right_tree)
    
    return __build(reverse_iterator(postorder))


preorder_construct = ["H","B","F",None,None,"E","A",None,None,None,"C",None,"D",None,"G","I",None, None,None]

postorder_construct = [
    None, None,  "F", None,None,"A",None,"E","B",None,None,None,None,"I",None,"G","D","C","H"
]


def tree_leaves(tree):
    if not tree:
        return []
    elif not tree.left and not tree.right:
        return [tree]
    else:
        return tree_leaves(tree.left) + tree_leaves(tree.right)