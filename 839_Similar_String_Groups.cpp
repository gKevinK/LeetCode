class Solution {
    bool similar(string & a, string & b) {
        int n = 0;
        for (int i = 0; i < a.size(); ++i)
            if (a[i] != b[i] && ++n > 2)
                return false;
        return true;
    }
public:
    int numSimilarGroups(vector<string>& A) {
        int n = A.size();
        vector<int> parent, level;
        for (int i = 0; i < n; ++i) {
            parent.push_back(i);
            level.push_back(0);
            for (int j = 0; j < i; ++j) {
                if (similar(A[i], A[j])) {
                    int ia = i, ib = j;
                    while (parent[ia] != ia) ia = parent[ia];
                    while (parent[ib] != ib) ib = parent[ib];
                    if (ia != ib) {
                        if (level[ia] < level[ib]) swap(ia, ib);
                        parent[ib] = ia;
                        level[ia] = max(level[ia], level[ib] + 1);
                    }
                }
            }
        }
        int r = 0;
        for (int i = 0; i < n; ++i) {
            if (parent[i] == i) {
                ++r;
            }
        }
        return r;
    }
};