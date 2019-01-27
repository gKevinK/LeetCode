class Solution {
public:
    int maxTurbulenceSize(vector<int>& A) {
        int last = 0, len = 0, res = 0;
        for (int i = 1; i < A.size(); i++) {
            int sign = A[i] == A[i - 1] ? 0 : (A[i] > A[i - 1] ? 1 : -1);
            if (sign * last < 0)
                len++;
            else
                len = sign == 0 ? 0 : 1;
            res = max(res, len);
            last = sign;
        }
        return res + 1;
    }
};