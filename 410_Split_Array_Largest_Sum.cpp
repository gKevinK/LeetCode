class Solution {
    bool isValid(vector<int>& nums, int m, int sum) {
        int s = 0;
        for (int & i : nums) {
            if (s + i > sum) {
                s = 0;
                if (--m <= 0)
                    return false;
            }
            s += i;
        }
        return true;
    }
public:
    int splitArray(vector<int>& nums, int m) {
        int lo = 0, hi = 0;
        for (int & n : nums) {
            lo = max(lo, n);
            hi += n;
        }
        while (lo < hi) {
            int mi = lo + (hi - lo) / 2;
            if (isValid(nums, m, mi)) hi = mi;
            else lo = mi + 1;
        }
        return lo;
    }
};