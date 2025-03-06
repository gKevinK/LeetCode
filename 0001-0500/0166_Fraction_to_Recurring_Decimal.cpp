class Solution {
public:
    string fractionToDecimal(int numerator, int denominator) {
        int a = numerator, b = denominator;
        unordered_map<int, int> m;
        if (a == 0) return "0";
        bool sgn = (a > 0) ^ (b > 0);
        string s = to_string((long long)a / b);
        if (sgn && s[0] != '-') s = "-" + s;
        a = (long long)a % b;
        if (a == 0) return s;
        s.push_back('.');
        for (int c = s.size(); a; c++) {
            if (m.find(a) != m.end()) {
                s = s + " )";
                for (int i = s.size() - 2; i >= m[a]; i--)
                    s[i] = s[i - 1];
                s[m[a]] = '(';
                return s;
            }
            m[a] = c;
            long long al = (long long)a * 10;
            s.push_back('0' + al / b * (sgn ? -1 : 1));
            a = al % b;
        }
        return s;
    }
};