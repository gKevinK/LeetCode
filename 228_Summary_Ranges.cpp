class Solution {
public:
    vector<string> summaryRanges(vector<int>& nums) {
        vector<string> v;
        int a = 0, b = -1, n = nums.size();
        for (int i = 0; i < n; i++) {
            if (a > b)
                a = b = nums[i];
            else if (b + 1 < nums[i]) {
                v.push_back(a == b ? to_string(a) : to_string(a) + "->" + to_string(b));
                a = b = nums[i];
            } else
                b++;
        }
        if (a <= b)
            v.push_back(a == b ? to_string(a) : to_string(a) + "->" + to_string(b));
        return v;
    }
};