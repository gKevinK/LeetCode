class Solution {
public:
    vector<int> constructArray(int n, int k) {
        vector<int> r;
        for (int i = n; i > k; --i)
            r.push_back(i);
        for (int i = 1, j = k; i <= j; ++i, --j) {
            r.push_back(i);
            if (i != j)
                r.push_back(j);
        }
        return r;
    }
};