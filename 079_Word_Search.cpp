class Solution {
    bool f(int i, int j, int t, vector<vector<char>>& b, string& w) {
        if (i < 0 || i >= b.size() || j < 0 || j >= b[0].size() || b[i][j] != w[t]) return false;
        t++;
        if (t == w.size()) return true;
        char c = b[i][j];
        b[i][j] = 0;
        bool r = f(i - 1, j, t, b, w) || f(i + 1, j, t, b, w) || f(i, j - 1, t, b, w) || f(i, j + 1, t, b, w);
        b[i][j] = c;
        return r;
    }
public:
    bool exist(vector<vector<char>>& board, string word) {
        if (board.empty() || board[0].empty() || word.empty()) return false;
        for (int i = 0; i < board.size(); i++)
            for (int j = 0; j < board[0].size(); j++)
                if (board[i][j] == word[0] && f(i, j, 0, board, word))
                    return true;
        return false;
    }
};