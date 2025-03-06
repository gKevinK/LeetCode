class Solution {
public:
    vector<int> diStringMatch(string S) {
        int i = 0, j = S.size();
        vector<int> r;
        for (char & c : S)
            r.push_back(c == 'I' ? i++ : j--);
        r.push_back(i);
        return r;
    }
};