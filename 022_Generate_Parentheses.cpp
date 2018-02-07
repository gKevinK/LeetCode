class Solution {
public:
    vector<string> generateParenthesis(int n) {
        int a = 0, p = 0, r = 1;
        string s(n * 2, ' ');
        vector<string> v;
        while (true) {
            if (p == n * 2) {
                v.push_back(s);
                p--; r = 0;
            } else if (r == 1 && a < n) {
                s[p++] = '('; a++;
            } else if (r == 1) {
                s[p++] = ')';
            } else if (s[p] == '(' && (2 * a - p - 2) > 0) {
                s[p++] = ')'; a--; r = 1;
            } else if (s[p] == '(') {
                p--; a--;
            } else if (s[p] == ')') {
                p--;
            } else if (p == 0 && r == 0) break;
            else break;
        }
        return v;
    }
};