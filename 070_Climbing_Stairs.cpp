class Solution {
public:
    int climbStairs(int n) {
        int a = 1, b = 1, t;
        for (int i = 1; i < n; i++) {
            t = b;
            b = a + b;
            a = t;
        }
        return b;
    }
};