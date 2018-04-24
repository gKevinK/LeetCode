class Solution {
public:
    string shortestCompletingWord(string licensePlate, vector<string>& words) {
        vector<int> v1(26, 0);
        for (char c : licensePlate) {
            if (c >= 'A' && c <= 'Z')
                v1[c - 'A']++;
            else if (c >= 'a' && c <= 'z')
                v1[c - 'a']++;
        }
        string res = "                                         ";
        for (string & word : words) {
            if (res.size() <= word.size())
                continue;
            vector<int> v2(26, 0);
            for (char & c : word)
                v2[c - 'a']++;
            bool t = true;
            for (int i = 0; i < 26; i++)
                if (v1[i] > v2[i])
                    t = false;
            if (t)
                res = word;
        }
        return res;
    }
};