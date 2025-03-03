class Solution {
public:
    int lastRemaining(int n) {
        int r = 1, begin = 1, step = 1;
        while (begin + step <= n) {
            if (r % 2) {
                begin += step;
            } else {
                if ((n - begin) / step % 2 == 0)
                    begin += step;
            }
            step *= 2;
            r++;
        }
        return begin;
    }
};