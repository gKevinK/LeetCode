class Solution {
public:
    int findMaximumXOR(vector<int>& nums) {
        int res = 0;
        for (int i = 30; i >= 0; i--) {
            unordered_set<int> s;
            int m = (res >> i) | 1;
            for (int & n : nums) {
                if (s.find((n >> i) ^ m) != s.end()) {
                    res |= 1 << i;
                    break;
                }
                s.insert(n >> i);
            }
            
        }
        return res;
    }
};