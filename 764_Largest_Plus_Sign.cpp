class Solution {
public:
    int orderOfLargestPlusSign(int N, vector<vector<int>>& mines) {
        auto mine = vector<vector<int>>(N, vector<int>(N, 1));
        for (auto & z : mines) mine[z[0]][z[1]] = 0;
        auto m = vector<vector<vector<int>>>(2, vector<vector<int>>(N + 1, vector<int>(N + 1, 0)));
        auto v = vector<int>(N, 0);
        int r = 0;
        for (int i = N - 1; i >= 0; --i) {
            for (int j = N - 1; j >= 0; --j) {
                if (mine[i][j]) {
                    m[0][i][j] = m[0][i + 1][j] + 1;
                    m[1][i][j] = m[1][i][j + 1] + 1;
                }
            }
        }
        for (int i = 0; i < N; ++i) {
            int c = 0;
            for (int j = 0; j < N; ++j) {
                if (mine[i][j]) {
                    ++c;
                    ++v[j];
                    
                    r = max(r, min({ m[0][i][j], m[1][i][j], c, v[j] }));
                } else {
                    c = 0;
                    v[j] = 0;
                }
            }
        }
        return r;
    }
};