class Solution {
public:
    string replaceWords(vector<string>& dict, string sentence) {
        unordered_set<string> s(dict.begin(), dict.end());
        int n = sentence.size();
        string r = "";
        for (int i = 0; i < n; ++i) {
            int j = i + 1;
            for (; j < n && sentence[j] != ' '; ++j) {
                if (s.find(sentence.substr(i, j - i)) != s.end())
                    break;
            }
            r += sentence.substr(i, j - i);
            r += ' ';
            for (; j < n && sentence[j] != ' '; ++j);
            i = j;
        }
        r.pop_back();
        return r;
    }
};