class Solution {
public:
    int bestRotation(vector<int>& A) {
        int r = 0, s = 0, score = 0, n = A.size();
        vector<int> v(2 * n, 0);
        for (int i = 0; i < n; ++i) {
            if (i >= A[i]) {
                v[i - A[i]]++;
                score++;
            }
        }
        s = score;
        for (int i = 0; i < n - 1; ++i) {
            score += 1;
            v[i + n - A[i]]++;
            score -= v[i];
            if (s < score) {
                r = i + 1;
                s = score;
            }
        }
        return r;
    }
};