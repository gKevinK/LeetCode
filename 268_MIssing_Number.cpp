class Solution {
public:
    int missingNumber(vector<int>& nums) {
        int x = 0, y = 0, max = 0;
        for (auto i : nums) {
            x = x ^ i;
        }
        for (int i = 0; i <= nums.size(); i++) {
            y = y ^ i;
        }
        return x ^ y;
    }
};