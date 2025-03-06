class Solution {
public:
    int dominantIndex(vector<int>& nums) {
        if (nums.size() == 0) return -1;
        if (nums.size() == 1) return 0;
        int m1 = -1, m2 = -1;
        for (int i = 0; i < nums.size(); i++) {
            if (m1 == -1 || nums[i] > nums[m1]) {
                m2 = m1;
                m1 = i;
            } else if (m2 == -1 || nums[i] > nums[m2]) {
                m2 = i;
            }
        }
        return nums[m1] >= nums[m2] * 2 ? m1 : -1;
    }
};