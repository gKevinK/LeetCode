class Solution {
public:
    int numSubarrayProductLessThanK(vector<int>& nums, int k) {
        int count = 0, n = nums.size();
        for (int i = 0, j = 0, prod = 1; j < n;) {
            prod *= nums[j++];
            while (prod >= k && i < j)
                prod /= nums[i++];
            count += j - i;
        }
        return count;
    }
};