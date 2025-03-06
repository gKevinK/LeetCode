class Solution {
    double f(int A, int B, vector<vector<double>> & m) {
        if (A <= 0 && B <= 0) return 0.5;
        if (A <= 0) return 1.0;
        if (B <= 0) return 0.0;
        if (m[A][B] > 0) return m[A][B];
        m[A][B] = 0.25 * (f(A-4, B, m) + f(A-3, B-1, m) + f(A-2, B-2, m) + f(A-1, B-3, m));
        return m[A][B];
    }
public:
    double soupServings(int N) {
        if (N > 4800)
            return 1.0;
        int s = (N + 24) / 25;
        vector<vector<double>> vv(s + 1, vector<double>(s + 1, 0));
        return f(s, s, vv);
    }
};