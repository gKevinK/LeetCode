class Solution {
public:
    int maximumGap(vector<int>& nums) {
        int mi = INT_MAX, ma = INT_MIN, n = nums.size();
        if (n <= 1) return 0;
        for (int & i : nums) {
            mi = min(mi, i);
            ma = max(ma, i);
        }
        int width = max(1, (ma - mi) / n), gap = 0;
        vector<vector<int>> buckets((ma - mi) / width + 1);
        for (int & i : nums)
            buckets[(i - mi) / width].push_back(i);
        int rb = INT_MIN;
        for (auto & b : buckets) {
            if (b.empty()) continue;
            int lb2 = INT_MAX, rb2 = INT_MIN;
            for (int & i : b) {
                lb2 = min(lb2, i);
                rb2 = max(rb2, i);
            }
            if (rb != INT_MIN)
                gap = max(gap, lb2 - rb);
            rb = rb2;
        }
        return gap;
    }
};