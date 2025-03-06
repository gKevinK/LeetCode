class Solution {
public:
    string convert(string s, int numRows) {
        if (numRows == 1) return s;
        string a = "";
        int i, j, r;
        for (i = 0, r = 0; r < s.size(); ++i, r = i * (numRows - 1) * 2)
            a.push_back(s[r]);
        for (j = 1; j < numRows - 1; ++j)
            for (i = 0, r = j; r < s.size(); ++i, r = i * (numRows - 1) + j + i % 2 * (numRows - 1 - 2 * j))
                a.push_back(s[r]);
        for (i = 0, r = numRows - 1; r < s.size(); ++i, r = i * (numRows - 1) * 2 + numRows - 1)
            a.push_back(s[r]);
        return a;
    }
};