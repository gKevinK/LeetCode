class Solution {
public:
    int removeElement(vector<int>& nums, int val) {
        int i = 0, j = 0;
        for (; j < nums.size(); ++i, ++j) {
            if (nums[j] == val) --i;
            else nums[i] = nums[j];
        }
        return i;
    }
};