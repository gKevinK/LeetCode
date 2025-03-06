class Solution {
public:
    int arrangeCoins(int n) {
        int r = 0, c = 0;
        while (n - c > r)
            c += ++r;
        return r;
    }
};