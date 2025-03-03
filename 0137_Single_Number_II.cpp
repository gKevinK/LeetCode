class Solution {
public:
    int singleNumber(vector<int>& nums) {
        vector<int> b(32, 0);
        for (auto & n : nums) {
            int ii = n;
            for (int i = 0; i < 32; ++i) {
                b[i] += ii & 1;
                ii >>= 1;
                if (ii == 0) break;
            }
        }
        int r = 0;
        for (int i = 31; i >= 0; --i) {
            r <<= 1;
            r += b[i] % 3;
        }
        return r;
    }
};