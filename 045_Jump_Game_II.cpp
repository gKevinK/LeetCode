class Solution {
public:
    int jump(vector<int>& nums) {
        if (nums.size() <= 1) return 0;
        int i = 0, m1 = 0, m2 = 0, d = 1;
        for (; i <= m2; d++) {
            for (; i <= m2; i++) {
                m1 = max(m1, i + nums[i]);
                if (m1 >= nums.size() - 1) return d;
            }
            m2 = m1;
        }
        return 0;
    }
};