class Solution {
public:
    int findUnsortedSubarray(vector<int>& nums) {
        int left = 0, right = -1, m = INT_MIN;
        for (int i = 0; i < nums.size(); i++) {
            if (nums[i] > m)
                m = nums[i];
            else if (nums[i] < m)
                right = i;
        }
        m = INT_MAX;
        for (int i = nums.size() - 1; i >= 0; i--) {
            if (nums[i] < m)
                m = nums[i];
            else if (nums[i] > m)
                left = i;
        }
        return right - left + 1;
    }
};