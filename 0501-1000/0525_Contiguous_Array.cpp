class Solution {
public:
    int findMaxLength(vector<int>& nums) {
        stack<int> s1, s2;
        int res = 0, r0 = 0;
        for (int i = 0; i < nums.size(); i++) {
            if (nums[i] == 1) {
                s1.push(r0);
                if (s2.empty())
                    r0 = i + 1;
                else {
                    r0 = s2.top();
                    s2.pop();
                    res = max(res, i - r0 + 1);
                }
            } else {
                s2.push(r0);
                if (s1.empty())
                    r0 = i + 1;
                else {
                    r0 = s1.top();
                    s1.pop();
                    res = max(res, i - r0 + 1);
                }
            }
        }
        return res;
    }
};