def house_robber(nums):
    length = len(nums)
    
    if length == 0:
        return 0
    
    if length == 1:
        return nums[0]
    
    nums[1] = max(nums[1], nums[0])
    
    for i in range(2, length):
        if nums[i-1] > (nums[i-2] + nums[i]):
            nums[i] = nums[i-1] 
        else:
            nums[i] += nums[i-2]
    
    return nums[length-1]
