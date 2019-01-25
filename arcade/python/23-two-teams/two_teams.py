def twoTeams(students):
    return sum([-j if i & 1 == 1 else j for i, j in enumerate(students)])

