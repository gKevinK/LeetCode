class Solution {
public:
    vector<string> findRelativeRanks(vector<int>& nums) {
        vector<string> r(nums.size());
        vector<int> v(nums.size());
        iota(v.begin(), v.end(), 0);
        sort(v.begin(), v.end(), [&nums](int a, int b) { return nums[a] > nums[b]; });
        for (int i = 0; i < v.size(); i++)
            r[v[i]] = i == 0 ? "Gold Medal" : (i == 1 ? "Silver Medal" : (i == 2 ? "Bronze Medal" : to_string(i + 1)));
        return r;
    }
};