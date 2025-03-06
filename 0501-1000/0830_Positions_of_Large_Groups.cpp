class Solution {
public:
    vector<vector<int>> largeGroupPositions(string S) {
        vector<vector<int>> r;
        int l = 0;
        for (int i = 1; i < S.size(); i++) {
            if (S[i] != S[i - 1]) {
                if (i - l >= 3)
                    r.push_back({ l, i - 1 });
                l = i;
            }
        }
        if (S.size() - l >= 3)
            r.push_back({ l, S.size() - 1 });
        return r;
    }
};