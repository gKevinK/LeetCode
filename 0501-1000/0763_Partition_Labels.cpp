class Solution {
public:
    vector<int> partitionLabels(string S) {
        vector<int> v(26, 0), res;
        for (int i = 0; i < S.size(); i++) {
            v[S[i] - 'a'] = max(v[S[i] - 'a'], i + 1);
        }
        int l = 0, r = 0;
        for (int i = 0; i < S.size(); i++) {
            r = max(v[S[i] - 'a'], r);
            if (i + 1 == r) {
                res.push_back(r - l);
                l = r;
                r = 0;
            }
        }
        return res;
    }
};