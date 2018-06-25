class Solution {
public:
    int scoreOfParentheses(string S) {
        vector<int> v = { 0 };
        for (char & c : S) {
            if (c == '(') {
                v.push_back(0);
            } else {
                int t = v.back();
                v.pop_back();
                v.back() += (t == 0) ? 1 : t * 2;
            }
        }
        return v[0];
    }
};