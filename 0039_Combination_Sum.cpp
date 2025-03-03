class Solution {
    void f(vector<int>& candidates, int target, vector<vector<int>>& res, vector<int>& cur, int loc) {
        cur.push_back(0);
        for (int i = loc; i < candidates.size() && candidates[i] <= target; i++) {
            cur.back() = candidates[i];
            if (candidates[i] == target) res.push_back(cur);
            else f(candidates, target - candidates[i], res, cur, i);
        }
        cur.pop_back();
    }
public:
    vector<vector<int>> combinationSum(vector<int>& candidates, int target) {
        vector<vector<int>> result;
        vector<int> current;
        sort(candidates.begin(), candidates.end());
        f(candidates, target, result, current, 0);
        return result;
    }
};