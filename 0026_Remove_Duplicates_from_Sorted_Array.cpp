class Solution {
public:
    int removeDuplicates(vector<int>& nums) {
        int c = 0, n = nums.size();
        for (int i = 1; i < n; ++i) {
            if (nums[i] == nums[i-1]) ++c;
            else nums[i-c] = nums[i];
        }
        return n - c;
    }
};