class Solution {
public:
    bool isSelfCrossing(vector<int>& x) {
        if (x.size() <= 3) return false;
        for (int i = 3; i < x.size(); i++) {
            int limit = INT_MAX;
            if (x[i - 3] >= x[i - 1])
                limit = x[i - 2];
            if (i >= 4 && x[i - 3] == x[i - 1])
                limit = x[i - 2] - x[i - 4];
            if (i >= 5 && x[i - 4] < x[i - 2] && x[i - 3] - x[i - 1] >= 0 && x[i - 5] >= x[i - 3] - x[i - 1])
                limit = x[i - 2] - x[i - 4];
            if (x[i] >= limit)
                return true;
        }
        return false;
    }
};