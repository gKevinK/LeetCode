class Solution {
public:
    int rotatedDigits(int N) {
        static vector<int> v1 = { 0, 1, 2, 3, 3, 3, 4, 5, 5, 6, 7 };
        static vector<int> v2 = { 0, 0, 0, 1, 1, 1, 2, 3, 3, 3, 4 };
        int r1 = 1, r2 = 0, t = N, c1 = 1, c2 = 0;
        while (t) {
            int d = t % 10;
            if (d == 3 || d == 4 || d == 7) r1 = r2 = 0;
            else if (d == 0 || d == 1 || d == 8) r2 = r2;
            else r2 = r1;
            r1 += v1[d] * c1;
            r2 += v2[d] * c1 + (v1[d] - v2[d]) * c2;
            c2 = c1 * 4 + c2 * 3;
            c1 = c1 * 7;
            t /= 10;
        }
        return r2;
    }
};