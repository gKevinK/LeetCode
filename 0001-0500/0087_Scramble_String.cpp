class Solution {
public:
    bool isScramble(string s1, string s2) {
        unordered_map<string, bool> m;
        return judge(m, s1, s2);
    }

private:
    bool judge(unordered_map<string, bool>& m, string s1, string s2) {
        int n = s1.size();
        if (s1 == s2)
            return true;
        if (n == 1)
            return false;

        array<int, 26> count{};
        for (int i = 0; i < n; i++) {
            count[s1[i] - 'a']++;
            count[s2[i] - 'a']--;
        }
        for (int i = 0; i < 26; i++) {
            if (count[i] != 0)
                return false;
        }

        string key = s1 + " " + s2;
        if (m.find(key) != m.end())
            return m[key];

        for (int i = 0; i + 1 < n; i++) {
            bool b1 = judge(m, s1.substr(0, i + 1), s2.substr(0, i + 1));
            bool b2 = judge(m, s1.substr(i + 1), s2.substr(i + 1));
            if (b1 && b2) {
                return m[key] = true;
            }
        }

        for (int i = 0; i + 1 < n; i++) {
            bool b1 = judge(m, s1.substr(0, i + 1), s2.substr(n - i - 1));
            bool b2 = judge(m, s1.substr(i + 1), s2.substr(0, n - i - 1));
            if (b1 && b2) {
                return m[key] = true;
            }
        }

        return m[key] = false;
    }
};