class Solution {
    set<vector<int>> f(vector<int>& c, int target, int hi) {
        if (target < 0) return { };
        if (hi == 0) {
            if (target == 0) return { { } };
            else return { };
        }
        set<vector<int>> res = f(c, target, hi - 1);
        for (const auto & v : f(c, target - c[hi - 1], hi - 1)) {
            vector<int> v1 = v;
            v1.push_back(c[hi - 1]);
            res.insert(v1);
        }
        return res;
    }
public:
    vector<vector<int>> combinationSum2(vector<int>& candidates, int target) {
        sort(candidates.begin(), candidates.end());
        set<vector<int>> s = f(candidates, target, candidates.size());
        return vector<vector<int>>(s.begin(), s.end());
    }
};