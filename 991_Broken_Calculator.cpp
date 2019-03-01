class Solution {
public:
    int brokenCalc(int X, int Y) {
        int r = 0;
        while (X < Y) {
            X <<= 1;
            r++;
        }
        for (int i = r, d = X - Y; i >= 0 && d; i--) {
            r += d / (1 << i);
            d %= (1 << i);
        }
        return r;
    }
};