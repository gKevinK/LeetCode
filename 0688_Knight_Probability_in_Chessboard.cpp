class Solution {
    int _N, _n;
    inline int g(int x) {
        return x >= _n ? _N - x - 1 : x;
    }
public:
    double knightProbability(int N, int K, int r, int c) {
        _N = N;
        _n = (N + 1) / 2;
        vector<vector<double>> m1(_n, vector<double>(_n, 0.0)), m2(_n, vector<double>(_n, 1.0));
        static vector<vector<int>> l = { { 2, -1 }, { 2, 1 }, { -1, 2 }, { 1, 2 }, { -2, -1 }, { -2, 1 }, { -1, -2 }, { 1, -2 } };
        for (int i = K - 1; i >= 0; i--) {
            for (int j = 0; j < _n; j++) {
                for (int k = 0; k < _n; k++) {
                    // if ((j + k + i - r - c) % 2 == 0) {
                        double p = 0.0;
                        for (auto & v : l) {
                            int x = g(j + v[0]), y = g(k + v[1]);
                            if (x < 0 || y < 0)
                                continue;
                            p += m2[x][y];
                        }
                        m1[j][k] = p / 8;
                    // }
                }
            }
            m1.swap(m2);
        }
        return m2[g(r)][g(c)];
    }
};