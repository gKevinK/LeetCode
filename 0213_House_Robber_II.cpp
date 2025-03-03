class Solution {
public:
    int rob(vector<int>& nums) {
        if (nums.size() == 1) return nums[0];
        int a1 = 0, a2 = 0, t1, s1, s2;
        for (int i = 0; i + 1 < nums.size(); i++) {
            t1 = max(a1, nums[i] + a2);
            a2 = max(a1, a2);
            a1 = t1;
        }
        s1 = a1;
        a1 = a2 = 0;
        for (int i = 1; i < nums.size(); i++) {
            t1 = max(a1, nums[i] + a2);
            a2 = max(a1, a2);
            a1 = t1;
        }
        s2 = a1;
        return max(s1, s2);
    }
};