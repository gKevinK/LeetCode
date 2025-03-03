class Solution {
    string add(string a, string b) {
        int i = a.size() - 1, j = b.size() - 1, c = 0;
        string r;
        while (i >= 0 || j >= 0 || c > 0) {
            c += (i >= 0 ? a[i] - '0' : 0) + (j >= 0 ? b[j] - '0' : 0);
            r.push_back(c % 10 + '0');
            c /= 10;
            i--, j--;
        }
        return string(r.rbegin(), r.rend());
    }
    bool f(string n, string p1, string p2) {
        string t = add(p1, p2);
        // cout << n << " " << p1 << " " << p2 << " " << t << endl;
        if (n == t) return true;
        if (n.size() < t.size() || n.substr(0, t.size()) != t) return false;
        return f(n.substr(t.size()), p2, t);
    }
public:
    bool isAdditiveNumber(string num) {
        for (int i = 1; i - 1 < (num.size() / 2); i++) {
            if (num[0] == '0' && i == 2) break;
            string p1 = num.substr(0, i);
            for (int j = i + 1; j * 2 - i - 1 < num.size(); j++) {
                if (num[i] == '0' && j - i == 2) break;
                string p2 = num.substr(i, j - i);
                if (f(num.substr(j), p1, p2)) return true;
            }
        }
        return false;
    }
};