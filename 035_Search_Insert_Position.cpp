class Solution {
public:
    int searchInsert(vector<int>& nums, int target) {
        int i = 0;
        for (; i < nums.size(); ++i) {
            if (target <= nums[i]) break;
        }
        return i;
    }
};