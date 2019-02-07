def isTestSolvable(ids, k):
    digitSum = lambda q_id: sum([int(i) for i in str(q_id)])

    sm = 0
    for questionId in ids:
        sm += digitSum(questionId)
    return sm % k == 0
