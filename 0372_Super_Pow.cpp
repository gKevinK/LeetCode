class Solution {
public:
    int superPow(int a, vector<int>& b) {
        int x = 1;
        a %= 1337;
        for (int i = 0; i < b.size(); ++i) {
            int t2 = x * x % 1337;
            int t4 = t2 * t2 % 1337;
            x = t4 * t4 % 1337 * t2 % 1337;
            for (int j = 0; j < b[i]; ++j) {
                x *= a;
                x = x % 1337;
            }
        }
        x = x % 1337;
        return x;
    }
};