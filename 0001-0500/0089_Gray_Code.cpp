class Solution {
public:
    vector<int> grayCode(int n) {
        vector<int> a, b = { 0 };
        for (int i = 0; i < n; i++) {
            a.clear();
            a.swap(b);
            int h = 1 << i;
            for (int & j : a)
                b.push_back(j);
            for (int k = a.size() - 1; k >= 0; k--)
                b.push_back(h + a[k]);
        }
        return b;
    }
};