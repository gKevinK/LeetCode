class Solution {
public:
    int pivotIndex(vector<int>& nums) {
        int ls = 0, rs = 0;
        for (int & i : nums)
            rs += i;
        for (int i = 0; i < nums.size(); i++) {
            rs -= nums[i];
            if (i > 0)
                ls += nums[i - 1];
            if (ls == rs)
                return i;
        }
        return -1;
    }
};