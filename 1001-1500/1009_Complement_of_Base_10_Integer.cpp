class Solution {
public:
    int bitwiseComplement(int N) {
        int r = 0, m = (1 << 30);
        if (N == 0) return 1;
        while ((N & m) == 0)
            m >>= 1;
        while (m) {
            r += m - (N & m);
            m >>= 1;
        }
        return r;
    }
};