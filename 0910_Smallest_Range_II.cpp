class Solution {
public:
    int smallestRangeII(vector<int>& A, int K) {
        int mi = *min_element(A.begin(), A.end());
        int ma = *max_element(A.begin(), A.end());
        int range = ma - mi, width = max(1, (ma - mi) / (int)A.size());
        if (K > range)
            return range;
        if (range > 4 * K)
            return range - 2 * K;
        
        int diff = range, rb = INT_MIN;
        vector<vector<int>> buckets((ma - mi) / width + 1);
        for (int & i : A)
            buckets[(i - mi) / width].push_back(i);
        for (auto & b : buckets) {
            if (b.empty()) continue;
            int lb2 = INT_MAX, rb2 = INT_MIN;
            for (int & i : b) {
                lb2 = min(lb2, i);
                rb2 = max(rb2, i);
            }
            if (rb != INT_MIN)
                diff = min(diff, max(rb + K, ma - K) - min(lb2 - K, mi + K));
            rb = rb2;
        }
        return diff;
    }
};