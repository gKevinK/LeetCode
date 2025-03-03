class Solution {
public:
    string mostCommonWord(string paragraph, vector<string>& banned) {
        unordered_set<string> bs(banned.begin(), banned.end());
        unordered_map<string, int> rs;
        for (char & c : paragraph)
            c = isalpha(c) ? tolower(c) : ' ';
        int N = paragraph.size();
        for (int i = 0; i < N; ) {
            int l = i;
            while (i < N && paragraph[i] != ' ')
                i++;
            string s = paragraph.substr(l, i - l);
            if (bs.find(s) == bs.end())
                rs[s]++;
            while (i < N && paragraph[i] == ' ')
                i++;
        }
        int n = 0;
        string s;
        for (auto & p : rs)
            if (p.second > n) {
                n = p.second;
                s = p.first;
            }
        return s;
    }
};