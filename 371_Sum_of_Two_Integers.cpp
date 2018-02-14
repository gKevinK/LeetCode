class Solution {
public:
    int getSum(int a, int b) {
        int r1 = a;
        int r2 = b;
        while (r2 != 0) {
            a = r1;
            b = r2;
            r1 = a ^ b;
            r2 = (a & b) << 1;
        }
        return r1;
    }
};