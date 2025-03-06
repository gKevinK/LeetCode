class Solution {
public:
    vector<string> commonChars(vector<string>& A) {
        vector<int> v(26, INT_MAX);
        for (string & s : A) {
            vector<int> v2(26, 0);
            for (char & c : s)
                v2[c - 'a']++;
            for (int i = 0; i < 26; i++)
                v[i] = min(v[i], v2[i]);
        }
        vector<string> r;
        for (int i = 0; i < 26; i++)
            for (int j = 0; j < v[i]; j++)
                r.push_back(string(1, 'a' + i));
        return r;
    }
};