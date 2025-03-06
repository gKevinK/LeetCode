class Solution {
public:
    int flipgame(vector<int>& fronts, vector<int>& backs) {
        unordered_set<int> s1, s2;
        for (int i = 0; i < fronts.size(); i++) {
            if (fronts[i] == backs[i]) {
                s2.insert(fronts[i]);   
            } else {
                s1.insert(fronts[i]);
                s1.insert(backs[i]);
            }
        }
        int r = INT_MAX;
        for (const int & i : s1) {
            if (i < r && s2.find(i) == s2.end())
                r = i;
        }
        return r == INT_MAX ? 0 : r;
    }
};