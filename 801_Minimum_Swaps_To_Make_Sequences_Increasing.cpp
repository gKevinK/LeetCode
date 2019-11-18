class Solution {
public:
    int minSwap(vector<int>& A, vector<int>& B) {
        int n = A.size(), p1 = -1, p2 = -1, s1 = 0, s2 = 0;
        for (int i = 0; i < n; ++i) {
            int ts1 = n, ts2 = n;
            if (A[i] > p1 && B[i] > p2) {
                ts1 = min(ts1, s1);
                ts2 = min(ts2, s2 + 1);
            }
            if (A[i] > p2 && B[i] > p1) {
                ts1 = min(ts1, s2);
                ts2 = min(ts2, s1 + 1);
            }
            s1 = ts1;
            s2 = ts2;
            p1 = A[i];
            p2 = B[i];
        }
        return min(s1, s2);
    }
};