class Solution {
public:
    int findTargetSumWays(vector<int>& nums, int S) {
        int sum = 0;
        map<int, int> m;
        m[0] = 1;
        for (int & i : nums) {
            sum += i;
            int d = i * 2;
            vector<int> v;            
            for (auto & p : m)
                v.push_back(p.first);
            for (int & k : v)
                m[k - d] = m[k - d] + m[k];
        }
        return m[S - sum];
    }
};