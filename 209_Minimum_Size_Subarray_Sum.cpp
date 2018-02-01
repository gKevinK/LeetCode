class Solution {
public:
    int minSubArrayLen(int s, vector<int>& nums) {
        int a = 0, b = 0, l = 0, sum = 0;
        for (b = 1; b <= nums.size(); ++b) {
            sum += nums[b-1];
            if (sum >= s) break;
        }
        if (sum < s) return 0;
        l = b - a;
        for (; b <= nums.size(); sum += nums[b], ++b) {
            while (sum - nums[a] >= s) {
                sum -= nums[a++];
            }
            if (b - a < l) {
                l = b - a;
            }
        }
        return l;
    }
};