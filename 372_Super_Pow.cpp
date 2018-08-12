class Solution {
public:
    int superPow(int a, vector<int>& b) {
        int x = 1;
        a = a % 1337;
        for (int j = 0; j < b[0]; ++j) {
            x *= a;
            x = x % 1337;
        }
        for (int i = 1; i < b.size(); ++i) {
            x = x % 1337;
            int t = x;
            for (int i = 0; i < 9; ++i) {
                x = x * t;
                x = x % 1337;
            }
            for (int j = 0; j < b[i]; ++j) {
                x *= a;
                x = x % 1337;
            }
        }
        x = x % 1337;
        return x;
    }
};