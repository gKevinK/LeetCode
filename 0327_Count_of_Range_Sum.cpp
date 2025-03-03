class Solution {
    int f(vector<long>& sum, int lo, int hi, int lower, int upper) {
        if (hi - lo <= 1) return 0;
        int mi = (hi - lo) / 2 + lo;
        int count = f(sum, lo, mi, lower, upper);
        count += f(sum, mi, hi, lower, upper);
        for (int i = lo, j = lo, k = mi; k < hi; k++) {
            while (i < mi && sum[k] - sum[i] > upper) i++;
            while (j < mi && sum[k] - sum[j] >= lower) j++;
            count += j - i;
        }
        inplace_merge(sum.begin() + lo, sum.begin() + mi, sum.begin() + hi);
        return count;
    }
public:
    int countRangeSum(vector<int>& nums, int lower, int upper) {
        if (nums.empty()) return 0;
        vector<long> sums(nums.size() + 1);
        sums[0] = 0;
        for (int i = 1; i <= nums.size(); i++)
            sums[i] = sums[i - 1] + nums[i - 1];
        return f(sums, 0, nums.size() + 1, lower, upper);
    }
};