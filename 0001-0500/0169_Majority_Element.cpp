int majorityElement(int* nums, int numsSize) {
    int x, n = 0;
    for (int i = 0; i < numsSize; i++) {
        if (n == 0) {
            x = nums[i];
            n += 1;
        } else if (x == nums[i])
            n += 1;
        else
            n -= 1;
    }
    return x;
}