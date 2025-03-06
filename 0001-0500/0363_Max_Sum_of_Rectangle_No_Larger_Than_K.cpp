class Solution {
public:
    int maxSumSubmatrix(vector<vector<int>>& matrix, int k) {
        int m = matrix.size(), n = matrix[0].size(), res = INT_MIN;
        for (int left = 0; left < n; left++) {
            vector<int> s(m, 0);
            for (int right = left; right < n; right++){
                int sum = 0, tmp = INT_MIN;
                for (int i = 0; i < m; i++) {
                    s[i] += matrix[i][right];
                    sum += s[i];
                    tmp = max(tmp, sum);
                    sum = max(sum, 0);
                }
                if (tmp == k) return k;
                if (tmp < k) { res = max(res, tmp); continue; }
                set<int> ss = { 0 };
                sum = 0;
                for (int i = 0; i < m; i++) {
                    sum += s[i];
                    auto it = ss.lower_bound(sum - k);
                    if (it != ss.end()) res = max(res, sum - *it);
                    ss.insert(sum);
                }
                if (res == k) return k;
            }
        }
        return res;
    }
};