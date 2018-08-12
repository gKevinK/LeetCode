class Solution {
private:
    void setV(vector<vector<char>>& board, int i, int j, int m, int n) {
        vector<tuple<int, int>> a1, a2;
        vector<tuple<int, int>> &v1 = a1;
        vector<tuple<int, int>> &v2 = a2;
        if (board[i][j] != 'O') return;
        board[i][j] = 'V';
        v1.push_back(make_tuple(i, j));
        while (v1.size() > 0) {
            for (int k = 0; k < v1.size(); k++) {
                tie(i, j) = v1[k];
                if (i - 1 >= 0 && board[i-1][j] == 'O') {
                    board[i-1][j] = 'V';
                    v2.push_back(make_tuple(i-1, j));
                }
                if (i + 1 < m && board[i+1][j] == 'O') {
                    board[i+1][j] = 'V';
                    v2.push_back(make_tuple(i+1, j));
                }
                if (j - 1 >= 0 && board[i][j-1] == 'O') {
                    board[i][j-1] = 'V';
                    v2.push_back(make_tuple(i, j-1));
                }
                if (j + 1 < n && board[i][j+1] == 'O') {
                    board[i][j+1] = 'V';
                    v2.push_back(make_tuple(i, j+1));
                }
            }
            v1.clear();
            v1.swap(v2);
        }
    }
    
public:
    void solve(vector<vector<char>>& board) {
        int m = board.size();
        int n = m ? board[0].size() : 0;
        for (int i = 0; i < m; i++) {
            if (board[i][0] == 'O') {
                setV(board, i, 0, m, n);
            }
            if (board[i][n-1] == 'O') {
                setV(board, i, n-1, m, n);
            }
        }
        for (int j = 0; j < n; j++) {
            if (board[0][j] == 'O') {
                setV(board, 0, j, m, n);
            }
            if (board[m-1][j] == 'O') {
                setV(board, m-1, j, m, n);
            }
        }
        for (int i = 0; i < m; i++) {
            for (int j = 0; j < n; j++) {
                if (board[i][j] == 'V') {
                    board[i][j] = 'O';
                } else {
                    board[i][j] = 'X';
                }
            }
        }
    }
};