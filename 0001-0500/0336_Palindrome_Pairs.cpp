class Solution {
    bool isPalindrome(string & s) {
        for (int l = 0, r = s.size() - 1; l < r; l++, r--)
            if (s[l] != s[r])
                return false;
        return true;
    }
public:
    vector<vector<int>> palindromePairs(vector<string>& words) {
        unordered_map<string, int> m;
        vector<vector<int>> r;
        for (int i = 0; i < words.size(); i++) {
            string w = words[i];
            reverse(w.begin(), w.end());
            m[w] = i;
        }
        if (m.find("") != m.end())
            for (int i = 0; i < words.size(); i++)
                if (words[i] != "" && isPalindrome(words[i]))
                    r.push_back({ m[""], i });
        for (int i = 0; i < words.size(); i++) {
            string w = words[i];
            int len = w.size();
            for (int j = 0; j < len; j++) {
                string left = w.substr(0, j), right = w.substr(j, len - j);
                if (m.find(left) != m.end() && m[left] != i && isPalindrome(right))
                    r.push_back({ i, m[left] });
                if (m.find(right) != m.end() && m[right] != i && isPalindrome(left))
                    r.push_back({ m[right], i });
            }
        }
        return r;
    }
};