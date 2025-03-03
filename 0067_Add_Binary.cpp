class Solution {
public:
    string addBinary(string a, string b) {
        if (b.size() > a.size()) return addBinary(b, a);
        int i = 0, c = 0, bn = 0;
        string r = "", i1 = "1";
        r.resize(a.size(), ' ');
        while ((int)a.size() - (++i) >= 0) {
            bn = ((int)b.size() - i >= 0) ? (b[b.size()-i] - '0') : 0;
            r[a.size()-i] = (a[a.size()-i] - '0' + bn + c) % 2 + '0';
            c = (a[a.size()-i] - '0' + bn + c) / 2;
        }
        if (c == 1) {
            i1.append(r);
            r = i1;
        }
        return r;
    }
};