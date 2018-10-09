class Solution {
public:
    int wiggleMaxLength(vector<int>& nums) {
        if (nums.empty()) return 0;
        int up = 0;
        int num = 1;
        for (int i = 1; i < nums.size(); ++i) {
            if (nums[i] > nums[i - 1] && up != 1) {
                up = 1;
                num++;
            } else if (nums[i] < nums[i - 1] && up != -1) {
                up = -1;
                num++;
            }
        }
        return num;
    }
};