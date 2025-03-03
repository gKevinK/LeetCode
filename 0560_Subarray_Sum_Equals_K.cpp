class Solution {
public:
    int subarraySum(vector<int>& nums, int k) {
        int sum = 0, count = 0;
        unordered_map<int, int> m;
        m[0] = 1;
        for (int & i : nums) {
            sum += i;
            count += m[sum - k];
            m[sum]++;
        }
        return count;
    }
};