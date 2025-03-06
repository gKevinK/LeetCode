class Solution {
private:
    bool *p;
public:
    bool A(string& s1, int x1, string& s2, int x2, string& s3) {
        if (s1.length() == x1 && s2.length() == x2)
            return true;
        if (p[x1 * (s2.length()+1) + x2] == false)
            return false;
        bool f = false;
        if (x1 < s1.length() && s1[x1] == s3[x1 + x2])
            f = A(s1, x1+1, s2, x2, s3);
        if (f == true) return true;
        if (x2 < s2.length() && s2[x2] == s3[x1 + x2])
            f = A(s1, x1, s2, x2+1, s3);
        if (f == false)
            p[x1 * (s2.length()+1) + x2] = false;
        return f;
    }

    bool isInterleave(string s1, string s2, string s3) {
        if (s1.length() + s2.length() != s3.length()) return false;
        p = new bool[(s1.length()+1) * (s2.length()+1)];
        for (int i = 0; i < (s1.length()+1) * (s2.length()+1); i++) {
            p[i] = true;
        }
        bool r = A(s1, 0, s2, 0, s3);
        delete p;
        return r;
    }
};