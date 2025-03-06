class Solution {
public:
    int numPairsDivisibleBy60(vector<int>& time) {
        vector<int> v(60, 0);
        for (int & t : time)
            v[t % 60]++;
        int r = v[0] * (v[0] - 1) / 2 + v[30] * (v[30] - 1) / 2;
        for (int i = 1; i < 30; ++i)
            r += v[i] * v[60 - i];
        return r;
    }
};