class Solution {
public:
    bool find132pattern(vector<int>& nums) {
        int s = INT_MIN;
        vector<int> v;
        for (int i = nums.size() - 1; i >= 0; i--) {
            if (nums[i] < s) return true;
            int t = INT_MAX;
            while (!v.empty() && v.back() < nums[i]) {
                t = v.back();
                v.pop_back();
            }
            v.push_back(nums[i]);
            if (t < INT_MAX)
                s = max(s, t);
        }
        return false;
    }
};