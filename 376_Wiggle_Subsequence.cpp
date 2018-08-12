class Solution {
public:
    int wiggleMaxLength(vector<int>& nums) {
        if (nums.empty()) return 0;
        bool up;
        int num = 1, last = nums[0], i;
        for (i = 1; i < nums.size(); ++i) {
            if (nums[i] > last) {
                up = false;
                num++;
                last = nums[i];
                break;
            } else if (nums[i] < last) {
                up = true;
                num++;
                last = nums[i];
                break;
            }
        }
        for (++i; i < nums.size(); ++i) {
            if (nums[i] > last && up == true) {
                up = false;
                num++;
            } else if (nums[i] < last && up == false) {
                up = true;
                num++;
            }
            last = nums[i];
        }
        return num;
    }
};