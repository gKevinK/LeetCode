class Solution {
public:
    int findLongestChain(vector<vector<int>>& pairs) {
        sort(pairs.begin(), pairs.end(), [](auto & a, auto & b) {
            return a[1] != b[1] ? a[1] < b[1] : a[0] < b[0]; });
        int n = 0, last = INT_MIN;
        for (auto & p : pairs) {
            if (p[0] > last || n == 0) {
                n++;
                last = p[1];
            }
        }
        return n;
    }
};