// Slow
class Solution {
public:
    bool canPartition(vector<int>& nums) {
        int sum = 0;
        for (int & i : nums)
            sum += i;
        if (sum % 2)
            return false;
        sum /= 2;
        set<int> s = { 0 };
        for (int & i : nums) {
            vector<int> v;
            for (const int & si : s) {
                if (si + i == sum)
                    return true;
                if (si + i < sum)
                    v.push_back(si + i);
            }
            for (int & vi : v)
                s.insert(vi);
        }
        return false;
    }
};

// Improved
// TODO