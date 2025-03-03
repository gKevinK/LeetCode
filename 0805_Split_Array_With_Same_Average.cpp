// class Solution {
// public:
//     bool splitArraySameAverage(vector<int>& A) {
//         int n = A.size(), sum = accumulate(A.begin(), A.end(), 0);
//         sort(A.begin(), A.end());
//         unordered_set<int> s = { 0 };
//         for (int ii = 1; ii < n; ++ii) {
//             int i = A[ii];
//             vector<int> v;
//             for (const int & j : s) {
//                 v.push_back(j);
//             }
//             for (int & j : v) {
//                 int r = j + i * n - sum;
//                 if (r == 0)
//                     return true;
//                 if (r < 0 && r + (n - ii) * (A[n - 1] * n - sum) >= 0)
//                     s.insert(r);
//             }
//         }
//         return false;
//     }
// };

class Solution {
public:
    bool f(int i, int sum, int N, vector<int>& A) {
        if (N == 0) return sum == 0;
        if (A[i] * N > sum) return false;
        for (int j = i; j + N <= A.size(); ++j) {
            if (j > i && A[j] == A[j - 1])
                continue;
            if (f(j + 1, sum - A[j], N - 1, A))
                return true;
        }
        return false;
    }
    bool splitArraySameAverage(vector<int>& A) {
        int n = A.size(), sum = accumulate(A.begin(), A.end(), 0);
        sort(A.begin(), A.end());
        for (int len = 1; len <= n / 2; ++len) {
            if (sum * len % n)
                continue;
            if (f(0, sum * len / n, len, A))
                return true;
        }
        return false;
    }
};