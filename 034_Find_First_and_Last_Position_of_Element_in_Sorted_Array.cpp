class Solution {
public:
    vector<int> searchRange(vector<int>& nums, int target) {
        int n = nums.size();
        int lo = 0, hi = n, lo1 = 0, lo2 = 0;
        while (lo < hi) {
            int mi = (hi - lo) / 2 + lo;
            if (nums[mi] < target) lo = mi + 1;
            else hi = mi;
        }
        lo1 = lo;
        lo = 0;
        hi = n;
        while (lo < hi) {
            int mi = (hi - lo) / 2 + lo;
            if (nums[mi] <= target) lo = mi + 1;
            else hi = mi;
        }
        lo2 = lo;
        if (lo1 == lo2) return { -1, -1 };
        else return { lo1, lo2 - 1 };
    }
};