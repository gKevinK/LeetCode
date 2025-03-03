class Solution {
public:
    int minEatingSpeed(vector<int>& piles, int H) {
        int lo = 1, hi = -1;
        for (int & p : piles)
            hi = max(hi, p);
        while (lo < hi) {
            int mi = (hi - lo) / 2 + lo, h = 0;
            for (int & p : piles)
                h += (p - 1) / mi + 1;
            // cout << lo << " " << mi << " " << hi << " - " << h << endl;
            if (h <= H) hi = mi;
            else lo = mi + 1;
        }
        return lo;
    }
};