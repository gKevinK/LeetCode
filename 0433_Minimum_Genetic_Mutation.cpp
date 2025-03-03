class Solution {
    string base = "ACGT";
    
    int ladderLength(string beginWord, string endWord, vector<string>& wordList) {
        unordered_set<string> set(wordList.begin(), wordList.end());
        unordered_set<string> s1 { beginWord }, s2 { endWord };
        if (set.find(endWord) == set.end()) return -1;
        int d = 0;
        while (!s1.empty() && !s2.empty()) {
            if (s1.size() > s2.size())
                s1.swap(s2);
            unordered_set<string> tmp;
            for (const string & w : s1) {
                string s = w;
                set.erase(s);
                for (int i = 0; i < s.size(); i++) {
                    char c = s[i];
                    for (char j : base) {
                        s[i] = j;
                        if (s2.find(s) != s2.end())
                            return d + 1;
                        if (set.find(s) != set.end()) {
                            tmp.insert(s);
                            set.erase(s);
                        }
                    }
                    s[i] = c;
                }
            }
            s1.swap(tmp);
            d++;
        }
        return -1;
    }
public:
    int minMutation(string start, string end, vector<string>& bank) {
        return ladderLength(start, end, bank);
    }
};