class Solution {
public:
    int slidingPuzzle(vector<vector<int>>& board) {
        unordered_set<string> s;
        vector<string> v1, v2;
        vector<vector<int>> a = { { 1, 3 }, { 0, 2, 4 }, { 1, 5 }, { 0, 4 }, { 1, 3, 5 }, { 2, 4 } };
        string o = { board[0][0], board[0][1], board[0][2], board[1][0], board[1][1], board[1][2] }, t = "123450";
        for (char & c : o) c += '0';
        if (o == t) return 0;
        v1.push_back(o);
        s.insert(o);
        for (int r = 1; !v1.empty(); r++) {
            for (string & str : v1) {
                int pos0 = str.find('0');
                for (int pos1 : a[pos0]) {
                    string str1 = str;
                    swap(str1[pos0], str1[pos1]);
                    if (s.find(str1) == s.end()) {
                        if (str1 == t)
                            return r;
                        v2.push_back(str1);
                        s.insert(str1);
                    }
                }
            }
            v1.clear();
            v1.swap(v2);
        }
        return -1;
    }
};