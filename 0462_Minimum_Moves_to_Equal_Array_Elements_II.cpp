class Solution {
public:
    int minMoves2(vector<int>& nums) {
        sort(nums.begin(), nums.end());
        int c = nums.size();
        int t = c % 2 ? nums[c / 2] : (nums[c / 2 - 1] + nums[c / 2]) / 2;
        int sum = 0;
        for (int & i : nums)
            sum += abs(i - t);
        return sum;
    }
};