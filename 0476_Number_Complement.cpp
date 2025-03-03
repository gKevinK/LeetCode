class Solution {
public:
    int findComplement(int num) {
        int m = 0, n = num;
        while (n) {
            m <<= 1;
            m++;
            n >>= 1;
        }
        return (~num) & m;
    }
};