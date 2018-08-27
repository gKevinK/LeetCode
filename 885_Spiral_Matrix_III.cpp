class Solution {
public:
    vector<vector<int>> spiralMatrixIII(int R, int C, int r0, int c0) {
        vector<int> b = { r0, c0, r0, c0 }, pos = { r0, c0 };
        vector<vector<int>> res = { pos }, step = { { 0, 1 }, { 1, 0 }, { 0, -1 }, { -1, 0 } };
        int d = 0;
        while (res.size() < R * C) {
            pos[0] += step[d][0];
            pos[1] += step[d][1];
            if (0 <= pos[0] && pos[0] < R && 0 <= pos[1] && pos[1] < C) res.push_back(pos);
            if (b[0] < pos[0] || b[1] < pos[1] || b[2] > pos[0] || b[3] > pos[1]) {
                d = (d + 1) % 4;
                b[0] = max(b[0], pos[0]);
                b[1] = max(b[1], pos[1]);
                b[2] = min(b[2], pos[0]);
                b[3] = min(b[3], pos[1]);
            }
        }
        return res;
    }
};