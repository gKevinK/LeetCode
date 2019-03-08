class Solution {
    pair<int, int> read(string& exp, int& i) {
        int sign = 1, u = 0, d = 0;
        if (exp[i] == '-')
            sign = -1;
        if (!isdigit(exp[i]))
            i++;
        while (exp[i] != '/')
            u = u * 10 + exp[i++] - '0';
        i++;
        while (i < exp.size() && isdigit(exp[i]))
            d = d * 10 + exp[i++] - '0';
        return { sign * u, d };
    }
public:
    string fractionAddition(string expression) {
        int i = 0, u = 0, d = 1;
        while (i < expression.size()) {
            auto a = read(expression, i);
            u = u * a.second + d * a.first;
            d *= a.second;
            int g = abs(gcd(u, d));
            u /= g;
            d /= g;
        }
        return to_string(u) + "/" + to_string(d);
    }
};