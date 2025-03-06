class Solution {
public:
    vector<int> kthSmallestPrimeFraction(vector<int>& A, int K) {
        int n = A.size();
        double lo = 0, hi = 1;
        while (true) {
            double mi = (lo + hi) / 2;
            int c = 0, r = n;
            for (int i = n - 1; i >= 0; --i) {
                while (r > 0 && mi * A[i] < A[r - 1]) --r;
                c += r;
            }
            if (c == K) {
                r = n - 1;
                vector<int> res = { 1, A[r] };
                for (int i = n - 1; i >= 0; --i) {
                    while (r >= 0 && mi * A[i] < A[r]) --r;
                    if (r >= 0 && A[r] * res[1] > A[i] * res[0]) res = { A[r], A[i] };
                }
                return res;
            } else if (c < K) {
                lo = mi;
            } else {
                hi = mi;
            }
        }
        // return { 0, 0 };
    }
};