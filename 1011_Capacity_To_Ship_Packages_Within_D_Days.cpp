class Solution {
public:
    int shipWithinDays(vector<int>& weights, int D) {
        int sum = 0, m = 0;
        for (int & w : weights) {
            sum += w;
            m = max(m, w);
        }
        int lo = max(sum / D, m), hi = sum;
        while (lo < hi) {
            int mi = (hi - lo) / 2 + lo;
            int day = 1, rest = mi;
            for (int & w : weights) {
                if (rest < w) {
                    day++;
                    rest = mi;
                }
                rest -= w;
            }
            if (day > D) lo = mi + 1;
            else hi = mi;
        }
        return lo;
    }
};