class Solution {
public:
    bool buddyStrings(string A, string B) {
        if (A.size() != B.size()) return false;
        vector<int> vi;
        for (int i = 0; i < A.size(); i++) {
            if (A[i] != B[i])
                vi.push_back(i);
        }
        if (vi.size() == 0 && A.size() >= 2) {
            bool s[26] = { false };
            for (char & c : A) {
                if (s[c - 'a']) return true;
                else s[c - 'a'] = true;
            }
            return false;
        }
        if (vi.size() != 2) return false;
        if (A[vi[0]] == B[vi[1]] && A[vi[1]] == B[vi[0]]) return true;
        return false;
    }
};