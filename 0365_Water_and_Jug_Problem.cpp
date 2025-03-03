class Solution {
public:
    bool canMeasureWater(int x, int y, int z) {
        if (x + y < z) return false;
        if (z == 0) return true;
        return z % gcd(x, y) == 0;
    }

    int gcd(int a, int b) {
        return (b == 0 ? a : gcd(b, a % b));
    }
};