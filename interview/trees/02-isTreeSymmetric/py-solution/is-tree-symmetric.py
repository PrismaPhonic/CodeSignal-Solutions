#
# Definition for binary tree:
# class Tree(object):
#   def __init__(self, x):
#     self.value = x
#     self.left = None
#     self.right = None
from collections import deque

def is_tree_symmetric(t):
    d = deque()
    
    d.append([t])
    
    while len(d) > 0:
        level = d.popleft()
        
        if is_level_symmetric(level) == False:
            return False
        
        next_level = []
        
        for node in level:
            if node != None:
                next_level.append(node.left)
                next_level.append(node.right)
                
        if len(next_level) > 0:
            d.append(next_level)
            
    return True


def is_level_symmetric(level):
    length = len(level)
    
    for i in range(0, math.floor(length/2)):
        if level[i] == None:
            if level[length-1-i] != None:
                return False
        else:
            if level[length-1-i] == None:
                return False
            else:
                if level[i].value != level[length-1-i].value:
                    return False
                
    return True
            
        
        
        

