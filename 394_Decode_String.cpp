class Solution {
public:
    string decodeString(string s) {
        string r;
        for (int i = 0; i < s.size(); i++) {
            if (s[i] >= '0' && s[i] <= '9') {
                int n = 0, nb = 0;
                while (s[i] != '[') n = n * 10 + s[i++] - '0';
                // cout << n << endl;
                int j = ++i;
                for (; nb || s[j] != ']'; j++) {
                    if (s[j] == '[') nb++;
                    if (s[j] == ']') nb--;
                }
                // cout << j << endl;
                string ss = decodeString(s.substr(i, j - i));
                while (n--) r += ss;
                i = j;
            } else
                r.push_back(s[i]);
        }
        return r;
    }
};