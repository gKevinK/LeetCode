class Solution {
public:
    string baseNeg2(int N) {
        if (N == 0) return "0";
        string r;
        bool flag = false, neg = N < 0;
        while (N) {
            r += '0' + (N & 1);
            N >>= 1;
            if (flag && r.back() == '1')
                N++;
            flag = !flag;
        }
        if (neg)
            r += "-";
        return string(r.rbegin(), r.rend());
    }
};