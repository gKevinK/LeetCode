class Solution {
    // vector<int> parse(const string & s) {
    //     int a = 0, b = 0, lb = 1, c = 0, lc = 1, dot = 0, bracket = 0;
    //     for (const char & ch : s) {
    //         if (ch == '.') {
    //             dot = 1; continue;
    //         } else if (ch == '(') {
    //             bracket = 1; continue;
    //         } else if (ch == ')')
    //             break;
    //         if (bracket) {
    //             c = c * 10 + (ch - '0');
    //             lc *= 10;
    //         } else if (dot) {
    //             b = b * 10 + (ch - '0');
    //             lb *= 10;
    //         } else {
    //             a = a * 10 + (ch - '0');
    //         }
    //     }
    //     if (lc == 1) return { a * lb + b, lb };
    //     int x2 = (lc - 1) * lb, x1 = b * (lc - 1) + c, d = gcd(x1, x2);
    //     x1 /= d; x2 /= d;
    //     return { a * x2 + x1, x2 };
    // }
    double parse(const string & s) {
        auto i = s.find('(');
        if (i != string::npos) {
            string a = s.substr(0, i), b = s.substr(i + 1, s.size() - i - 2);
            for (int j = 0; j < 20; j++)
                a += b;
            return stod(a);
        } else
            return stod(s);
    }
public:
    bool isRationalEqual(string S, string T) {
        return parse(S) == parse(T);
    }
};