class Solution {
public:
    int longestMountain(vector<int>& A) {
        int m = 0, n = A.size();
        for (int i = 1; i < n - 1; i++) {
            if (A[i] > A[i - 1] && A[i] > A[i + 1]) {
                int l = i - 1, r = i + 1;
                while (l - 1 >= 0 && A[l - 1] < A[l])
                    l--;
                while (r + 1 < n && A[r + 1] < A[r])
                    r++;
                m = max(m, r - l + 1);
            }
        }
        return m;
    }
};