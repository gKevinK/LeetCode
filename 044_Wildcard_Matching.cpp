class Solution {
public:
    bool isMatch(string s, string p) {
        int si = 0, si2 = 0, pi = 0, star = -1, ss = s.size(), ps = p.size();
        while (si < ss) {
            if (pi < ps && (p[pi] == '?' || s[si] == p[pi])) {
                si++; pi++;
            }
            else if (pi < ps && p[pi] == '*') {
                star = pi++;
                si2 = si;
            }
            else if (star >= 0) {
                pi = star + 1;
                si = ++si2;
            }
            else return false;
        }
        while (pi < ps && p[pi] == '*') pi++;
        return pi == ps;
    }
};