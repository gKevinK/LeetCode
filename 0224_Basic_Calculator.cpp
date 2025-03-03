class Solution {
    int cal(string& s, int i, int j) {
        int r1 = 0, r2 = 0, n = 0;
        char op = '+';
        for (int ic = i; ic < j; ic++) {
            char c = s[ic];
            if (c == '(') {
                int ic2 = ic, pn = 1;
                while (pn) {
                    ic2++;
                    if (s[ic2] == '(') pn++;
                    if (s[ic2] == ')') pn--;
                }
                n = cal(s, ic + 1, ic2);
                ic = ic2;
            }
            if ('0' <= c && c <= '9')
                n = n * 10 + (c - '0');
            if (c == '+' || c == '-' || c == '*' || c == '/' || ic == j - 1) {
                switch (op) {
                    case '+': r2 += n; break;
                    case '-': r2 -= n; break;
                    case '*': r2 *= n; break;
                    case '/': r2 /= n; break;
                }
                n = 0;
                op = c;
                if (c == '+' || c == '-' || ic == j - 1) {
                    r1 += r2;
                    r2 = 0;
                }
            }
        }
        return r1;
    }
public:
    int calculate(string s) {
        return cal(s, 0, s.size());
    }
};