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
        vector<int> v(wordList.size(), -1);
        vector<int> v1, v2;
        int e = -1;
        for (int i = 0; i < wordList.size(); i++)
            if (wordList[i] == endWord) {
                e = i;
                break;
            }
        if (e == -1) return 0;
        for (int i = 0; i < wordList.size(); i++)
            if (link(beginWord, wordList[i])) {
                if (i == e) return 2;
                v[i] = 1;
                v1.push_back(i);
            }
        while (!v1.empty()) {
            for (int i : v1) {
                if (link(wordList[i], endWord))
                    return v[i] + 2;
                for (int j = 0; j < wordList.size(); j++) {
                    if (v[j] == -1 && link(wordList[i], wordList[j])) {
                        if (j == e) return v[i] + 2;
                        v[j] = v[i] + 1;
                        v2.push_back(j);
                    }
                }
            }
            v1.clear();
            v1.swap(v2);
        }
        return 0;
    }
};