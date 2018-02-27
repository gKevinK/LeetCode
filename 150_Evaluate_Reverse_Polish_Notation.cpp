class Solution {
public:
    int evalRPN(vector<string>& tokens) {
        stack<int> s;
        for (string & t : tokens) {
            if (t == "+" || t == "-" || t == "*" || t == "/") {
                int i2 = s.top(); s.pop();
                int i1 = s.top(); s.pop();
                int res = 0;
                if (t == "+")
                    res = i1 + i2;
                else if (t == "-")
                    res = i1 - i2;
                else if (t == "*")
                    res = i1 * i2;
                else
                    res = i1 / i2;
                s.push(res);
            } else {
                int i = 0, sgn = 1;
                for (char & c : t) {
                    if (c == '-') { 
                        sgn = -1;
                        continue;
                    }
                    i = i * 10 + (c - '0');
                }
                s.push(i * sgn);
            }
        }
        return s.top();
    }
};