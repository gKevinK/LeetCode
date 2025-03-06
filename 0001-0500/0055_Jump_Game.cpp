class Solution {
public:
    bool canJump(vector<int>& nums) {
        if (nums.size() == 1) return true;
        for (int i = nums.size() - 2; i >= 0; --i) {
            if (nums[i] == 0) {
                bool flag = false;
                for (int j = i - 1; j >= 0; --j) {
                    if (nums[j] > i - j) {
                        flag = true;
                        break;
                    }
                }
                if (flag == false) return false;
            }
        }
        return true;
    }
};