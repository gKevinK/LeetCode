class Solution {
    int f(vector<int>& A, int l, int r) {
        int m = l + (r - l) / 2;
        if (A[m - 1] < A[m] && A[m] < A[m + 1])
            return f(A, m, r);
        if (A[m - 1] > A[m] && A[m] > A[m + 1])
            return f(A, l, m);
        return m;
    }
public:
    int peakIndexInMountainArray(vector<int>& A) {
        return f(A, 0, A.size() - 1);
    }
};