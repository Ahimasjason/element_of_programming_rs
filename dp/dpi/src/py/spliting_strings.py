from collections import deque




def split_string_bfs(dictionary , sentence):
    splits = {"":[]}
    progress = deque([""])
    while progress:
        next_process = progress.popleft()
        if next_process == sentence:
            return splits[next_process]
        
        for w in dictionary:
            if sentence[len(next_process):].startswith(w):
                next_w = next_process + w
                if next_w  not in splits:
                    splits[next_w] = splits[next_process] + [w]
                    progress.append(next_w)
    

def dfs(dictionary, sentence):
    if not sentence:
        return []
    for word in dictionary:
        if sentence.startswith(word):
            r = dfs(dictionary, sentence[len(word):])
            if r is not None:
                return r + [word]
            
d = ["ice","cream" , "icecream"]
w = "icecreamicecream"