class Solution {
public:
    int checkRecord(int n) {
        int mod = 1'000'000'007, sum = 0;
        vector<int> v1(6, 0), v2(6, 0);
        v1[0] = 1;
        for (int i = 0; i < n; i++) {
            v2[0] = (v1[0] + v1[1]) % mod;
            v2[0] = (v2[0] + v1[2]) % mod;
            v2[1] = v1[0];
            v2[2] = v1[1];
            v2[3] = (v2[0] + v1[3]) % mod;
            v2[3] = (v2[3] + v1[4]) % mod;
            v2[3] = (v2[3] + v1[5]) % mod;
            v2[4] = v1[3];
            v2[5] = v1[4];
            v1.swap(v2);
        }
        for (int i : v1)
            sum = (sum + i) % mod;
        return sum;
    }
};