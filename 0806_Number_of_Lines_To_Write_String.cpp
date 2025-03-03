class Solution {
public:
    vector<int> numberOfLines(vector<int>& widths, string S) {
        int line = 1, len = 0;
        for (char & c : S) {
            len += widths[c - 'a'];
            if (len > 100) {
                len = widths[c - 'a'];
                line++;
            }
        }
        return vector<int> { line, len };
    }
};