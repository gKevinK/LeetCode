class Solution {
public:
    vector<string> letterCombinations(string digits) {
        vector<string> r = {""}, t;
        if (digits.size() == 0) return t;
        string charmap[] = { "", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz" };
        for (auto c : digits) {
            for (auto s : r) {
                for (auto c2 : charmap[c-'1']) {
                    t.push_back(s + c2);
                }
            }
            r.clear();
            swap(r, t);
        }
        return r;
    }
};