class Solution {
public:
    int totalFruit(vector<int>& tree) {
        if (tree.empty()) return 0;
        int i1 = 0, i2 = 0, i3 = 0, i4 = 0, n = tree.size();
        while (i2 < n && tree[i2] == tree[i1]) i2++;
        int res = i2 - i1;
        while (i2 < n) {
            i3 = i2;
            for (i4 = i2 + 1; i4 < n && (tree[i4] == tree[i1] || tree[i4] == tree[i2]); i4++) {
                if (tree[i4] != tree[i3])
                    i3 = i4;
            }
            res = max(res, i4 - i1);
            i1 = i3;
            i2 = i4;
        }
        return res;
    }
};