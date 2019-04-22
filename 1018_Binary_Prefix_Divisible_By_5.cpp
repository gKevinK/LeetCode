class Solution {
public:
    vector<bool> prefixesDivBy5(vector<int>& A) {
        vector<bool> r;
        int t = 0;
        for (int & i : A) {
            t = (t * 2 + i) % 5;
            r.push_back(t == 0);
        }
        return r;
    }
};