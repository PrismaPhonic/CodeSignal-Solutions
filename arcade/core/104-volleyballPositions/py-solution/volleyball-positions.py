def volleyballPositions(formation, k):
    for i in range(0, k % 6):
        rotate_counter(formation)
    
    return formation
    
    
def rotate_counter(formation):
    temp = formation[0][1]
    formation[0][1] = formation[1][2]
    formation[1][2] = formation[3][2]
    formation[3][2] = formation[2][1]
    formation[2][1] = formation[3][0]
    formation[3][0] = formation[1][0]
    formation[1][0] = temp
