class Solution {
public:
    int findMinDifference(vector<string>& timePoints) {
        vector<int> v(24 * 60, 0);
        int r = INT_MAX, begin = 24 * 60, last = -24 * 60;
        for (string & p : timePoints) {
            int t = stoi(p.substr(0, 2)) *  60 + stoi(p.substr(3, 2));
            if (v[t] == 1) return 0;
            v[t] = 1;
        }
        for (int i = 0; i < 24 * 60; i++) {
            if (v[i]) {
                r = min(r, i - last);
                begin = min(begin, i);
                last = i;
            }
        }
        return min(r, begin + 24 * 60 - last);
    }
};