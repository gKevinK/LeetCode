class Solution {
    vector<int> v;
public:
    Solution(vector<int> nums) {
        v = nums;
    }
    
    int pick(int target) {
        int k = 0, r = 0;
        for (int i = 0; i < v.size(); i++)
            if (v[i] == target && rand() % ++k == 0)
                r = i;
        return r;
    }
};

/**
 * Your Solution object will be instantiated and called as such:
 * Solution obj = new Solution(nums);
 * int param_1 = obj.pick(target);
 */