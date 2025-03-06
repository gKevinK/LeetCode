class Solution {
public:
    int hammingDistance(int x, int y) {
        int s = 0;
        while (x != y) {
            s += (x % 2 != y % 2);
            x >>= 1;
            y >>= 1;
        }
        return s;
    }
};