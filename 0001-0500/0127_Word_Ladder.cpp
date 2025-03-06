// class Solution {
//     bool link(const string & a, const string & b) {
//         int n = 0;
//         for (int i = 0; i < a.size(); i++)
//             if (a[i] != b[i])
//                 n++;
//         return n == 1;
//     }
// public:
//     int ladderLength(string beginWord, string endWord, vector<string>& wordList) {
//         vector<int> v(wordList.size(), -1);
//         vector<int> v1, v2;
//         int e = -1;
//         for (int i = 0; i < wordList.size(); i++)
//             if (wordList[i] == endWord) {
//                 e = i;
//                 break;
//             }
//         if (e == -1) return 0;
//         for (int i = 0; i < wordList.size(); i++)
//             if (link(beginWord, wordList[i])) {
//                 if (i == e) return 2;
//                 v[i] = 1;
//                 v1.push_back(i);
//             }
//         while (!v1.empty()) {
//             for (int i : v1) {
//                 if (link(wordList[i], endWord))
//                     return v[i] + 2;
//                 for (int j = 0; j < wordList.size(); j++) {
//                     if (v[j] == -1 && link(wordList[i], wordList[j])) {
//                         if (j == e) return v[i] + 2;
//                         v[j] = v[i] + 1;
//                         v2.push_back(j);
//                     }
//                 }
//             }
//             v1.clear();
//             v1.swap(v2);
//         }
//         return 0;
//     }
// };

class Solution {
    bool link(const string & a, const string & b) {
        int n = 0;
        for (int i = 0; i < a.size(); i++)
            if (a[i] != b[i])
                n++;
        return n == 1;
    }
public:
    int ladderLength(string beginWord, string endWord, vector<string>& wordList) {
        unordered_set<string> set(wordList.begin(), wordList.end());
        unordered_set<string> s1 { beginWord }, s2 { endWord };
        if (set.find(endWord) == set.end()) return 0;
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
                    for (char j = 'a'; j <= 'z'; j++) {
                        s[i] = j;
                        if (s2.find(s) != s2.end())
                            return d + 2;
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
        return 0;
    }
};