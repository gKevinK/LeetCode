class Solution {
public:
    bool repeatedSubstringPattern(string s) {
        int n = s.size();
        for (int i = 2; i <= n; i++) {
            if (n % i) continue;
            bool flag = true;
            int l = n / i;
            for (int j = l; j < n && flag; j += l)
                for (int k = 0; k < l && flag; k++)
                    if (s[k] != s[j + k])
                        flag = false;
            if (flag) return true;
        }
        return false;
    }
};