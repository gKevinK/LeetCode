class Solution {
public:
    int minSwapsCouples(vector<int>& row) {
        vector<int> pos(row.size());
        for (int i = 0; i < row.size(); ++i)
            pos[row[i]] = i;
        int r = 0;
        for (int i = 0; i < row.size(); i += 2) {
            if (row[i] / 2 != row[i + 1] / 2) {
                int t = pos[row[i + 1] / 2 * 4 + 1 - row[i + 1]];
                swap(row[i], row[t]);
                swap(pos[row[i]], pos[row[t]]);
                ++r;
            }
        }
        return r;
    }
};