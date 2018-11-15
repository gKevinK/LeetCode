class Solution {
public:
    vector<int> nextGreaterElement(vector<int>& findNums, vector<int>& nums) {
        unordered_map<int, int> m;
        vector<int> s, res;
        for (int i = nums.size() - 1; i >= 0; i--) {
            while (!s.empty() && nums[i] >= s.back())
                s.pop_back();
            m[nums[i]] = s.empty() ? -1 : s.back();
            s.push_back(nums[i]);
        }
        for (int & n : findNums)
            res.push_back(m[n]);
        return res;
    }
};