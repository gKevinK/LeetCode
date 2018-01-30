class Solution {
public:
    int longestValidParentheses(string s) {
        stack<int> st;
        int res = 0, r0 = 0;
        for (int i = 0; i < s.size(); i++) {
            if (s[i] == '(') {
                st.push(r0);
                r0 = i + 1;
            } else {
                if (st.empty()) {
                    r0 = i + 1;
                    continue;
                }
                r0 = st.top();
                st.pop();
                res = max(res, i - r0 + 1);
            }
        }
        return res;
    }
};