class Solution {
public:
    int smallestDistancePair(vector<int>& nums, int k) {
        sort(nums.begin(), nums.end());
        int lo = 0, hi = 1'000'001, n = nums.size();
        while (lo < hi) {
            int mi = (hi - lo) / 2 + lo;
            int c = 0, j = 1;
            for (int i = 0; i < n - 1; ++i) {
                while (j < n && nums[j] - nums[i] <= mi) ++j;
                c += j - i - 1;
            }
            if (c >= k) hi = mi;
            else lo = mi + 1;
        }
        return lo;
    }
};