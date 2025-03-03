class Solution {
    int idx(int i, int n) {
        return (i * 2 + 1) % (n | 1);
    }
public:
    void wiggleSort(vector<int>& nums) {
        int n = nums.size();
        if (n <= 1) return;
        auto mp = nums.begin() + n / 2;
        nth_element(nums.begin(), mp, nums.end());
        int mi = * mp;
        int i = 0, j = 0, k = n - 1;
        while (j <= k) {
            if (nums[idx(j, n)] > mi) {
                swap(nums[idx(i++, n)], nums[idx(j++, n)]);
            } else if (nums[idx(j, n)] < mi) {
                swap(nums[idx(j, n)], nums[idx(k--, n)]);
            } else
                j++;
        }
    }
};