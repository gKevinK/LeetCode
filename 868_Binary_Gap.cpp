class Solution {
public:
    int binaryGap(int N) {
        int d = 0, c = -1000;
        while (N) {
            if (N % 2) {
                d = max(d, c);
                c = 0;
            }
            N /= 2;
            c++;
        }
        return d;
    }
};