class Solution {
public:
    vector<string> findRepeatedDnaSequences(string s) {
        unordered_set<string> s1, s2;
        for (int i = 0; i + 10 <= s.size(); i++) {
            string ss = s.substr(i, 10);
            if (s1.find(ss) != s1.end())
                s2.insert(ss);
            else
                s1.insert(ss);
        }
        return vector<string>(s2.begin(), s2.end());
    }
};