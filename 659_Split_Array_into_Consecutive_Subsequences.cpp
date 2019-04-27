class Solution {
public:
    bool isPossible(vector<int>& nums) {
        int v1 = 0, v2 = 0, v3 = 0, n = nums.size();
        for (int i = 0, j = 1; i < n; i = j++) {
            if (i > 0 && nums[i] > nums[i - 1] + 1) {
                if (v1 || v2) return false;
                v1 = v2 = v3 = 0;
            }
            while (j < n && nums[j] == nums[i]) j++;
            int c = j - i;
            if (c < v1 + v2) return false;
            v3 = v2 + min(v3, c - v1 - v2);
            v2 = v1;
            v1 = c - v2 - v3;
        }
        return v1 == 0 && v2 == 0;
    }
};