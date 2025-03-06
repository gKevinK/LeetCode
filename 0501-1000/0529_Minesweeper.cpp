class Solution {
public:
    vector<vector<char>> updateBoard(vector<vector<char>>& board, vector<int>& click) {
        int m = board.size(), n = board[0].size();
        if (board[click[0]][click[1]] == 'M') {
            board[click[0]][click[1]] = 'X';
        } else {
            queue<pair<int, int>> q;
            q.push({ click[0], click[1] });
            while (!q.empty()) {
                int x = q.front().first, y = q.front().second, c = 0;
                q.pop();
                for (int i : { -1, 0, 1 })
                    for (int j : { -1, 0, 1 }) {
                        int x1 = x + i, y1 = y + j;
                        if ((i == 0 && j == 0) || x1 < 0 || x1 >= m || y1 < 0 || y1 >= n)
                            continue;
                        c += board[x1][y1] == 'M';
                    }
                board[x][y] = 'B';
                if (c) {
                    board[x][y] = '0' + c;
                } else {
                    for (int i : { -1, 0, 1 })
                        for (int j : { -1, 0, 1 }) {
                            int x1 = x + i, y1 = y + j;
                            if ((i == 0 && j == 0) || x1 < 0 || x1 >= m || y1 < 0 || y1 >= n)
                                continue;
                            if (board[x1][y1] == 'E') {
                                board[x1][y1] = 'W';
                                q.push({ x1, y1 });
                            }
                        }
                }
            }
        }
        return board;
    }
};