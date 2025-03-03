class Solution {
public:
    vector<vector<int>> subsetsWithDup(vector<int>& nums) {
        sort(nums.begin(), nums.end());
        vector<vector<int>> r = { {} };
        for (int i = 0, j = 1; i < nums.size(); i = j++) {
            while (j < nums.size() && nums[j] == nums[j - 1]) j++;
            int size = r.size();
            for (int k = 0; k < j - i; k++) {
                for (int l = 0; l < size; l++) {
                    r.push_back(r[k * size + l]);
                    r.back().push_back(nums[i]);
                }
            }
        }
        return r;
    }
};