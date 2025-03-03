class Solution {
    string read_name(string & expr, int & pos) {
        string r = "";
        while (pos < expr.size() && expr[pos] != ' ' && expr[pos] != ')')
            r += expr[pos++];
        return r;
    }
    int read_int(string & expr, int & pos) {
        int r = 0;
        int sign = 1;
        if (expr[pos] == '-') {
            sign = -1;
            ++pos;
        }
        while (pos < expr.size() && expr[pos] != ' ' && expr[pos] != ')')
            r = r * 10 + (expr[pos++] - '0');
        return sign * r;
    }
    int eval(string & expr, int & pos, unordered_map<string, int> & m) {
        if (expr[pos] == '(') {
            ++pos;
            string cmd = read_name(expr, pos);
            ++pos;
            int r = 0;
            if (cmd == "let") {
                unordered_map<string, int> m2 = m;
                while (true) {
                    if (isalpha(expr[pos])) {
                        string n = read_name(expr, pos);
                        if (expr[pos] == ')') {
                            r = m2[n];
                            break;
                        }
                        ++pos;
                        int v = eval(expr, pos, m2);
                        ++pos;
                        m2[n] = v;
                    } else {
                        r = eval(expr, pos, m2);
                        break;
                    }
                }
            } else {
                int o1 = eval(expr, pos, m);
                ++pos;
                int o2 = eval(expr, pos, m);
                r = (cmd == "add") ? o1 + o2 : o1 * o2;
            }
            ++pos;
            return r;
        } else if (isalpha(expr[pos])) {
            return m[read_name(expr, pos)];
        } else {
            return read_int(expr, pos);
        }
    }
public:
    int evaluate(string expression) {
        unordered_map<string, int> m;
        int pos = 0;
        return eval(expression, pos, m);
    }
};