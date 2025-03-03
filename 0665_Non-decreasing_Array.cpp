class Solution {
public:
    bool checkPossibility(vector<int>& nums) {
        bool m = false;
        int n = nums.size();
        for (int i = 0; i < n - 1; i++) {
            if (nums[i] <= nums[i + 1]) continue;
            if (m) return false;
            m = true;
            if ((i - 1 >= 0 && nums[i - 1] > nums[i + 1]) && (i + 2 < n && nums[i] > nums[i + 2])) return false;
        }
        return true;
    }
};