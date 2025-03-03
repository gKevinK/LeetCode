class Solution {
public:
    int maxProduct(vector<int>& nums) {
        int po = nums[0], ne = nums[0], ma = nums[0];
        for (int i = 1; i < nums.size(); ++i) {
            int t  = po;
            po = max(po * nums[i], max(ne * nums[i], nums[i]));
            ne = min(ne * nums[i], min(t * nums[i], nums[i]));
            ma = max(ma, po);
        }
        return ma;
    }
};