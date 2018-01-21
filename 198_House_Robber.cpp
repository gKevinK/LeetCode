class Solution {
public:
    int rob(vector<int>& nums) {
        int a1 = 0, a2 = 0, t1;
        for (int & i : nums) {
            t1 = max(a1, i + a2);
            a2 = max(a1, a2);
            a1 = t1;
        }
        return a1;
    }
};