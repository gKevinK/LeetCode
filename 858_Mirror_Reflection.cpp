class Solution {
public:
    int mirrorReflection(int p, int q) {
        int a = p, b = q;
        while (a % b) {
            int t = a % b;
            a = b;
            b = t;
        }
        p /= b;
        q /= b;
        if (q % 2)
            return p % 2 ? 1 : 2;
        else
            return 0;
    }
};