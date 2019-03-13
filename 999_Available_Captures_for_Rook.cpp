class Solution {
    pair<int, int> rook(vector<vector<char>>& b) {
        for (int i = 0; i < b.size(); i++)
            for (int j = 0; j < b[i].size(); j++)
                if (b[i][j] == 'R')
                    return { i, j };
        return { -1, -1 };
    }
public:
    int numRookCaptures(vector<vector<char>>& board) {
        static vector<vector<int>> dir = { { -1, 0 }, { 1, 0 }, { 0, -1 }, { 0, 1 } };
        auto r = rook(board);
        int count = 0, m = board.size(), n = board[0].size();
        for (auto & d : dir) {
            int x = r.first + d[0], y = r.second + d[1];
            for (; x >= 0 && x < m && y >= 0 && y < n; x += d[0], y += d[1]) {
                if (board[x][y] == 'p') {
                    count++;
                    break;
                } else if (board[x][y] == 'B')
                    break;
            }
        }
        return count;
    }
};