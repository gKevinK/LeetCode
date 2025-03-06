class Solution {
public:
    bool isValid(string S) {
        vector<int> v;
        for (char & c : S) {
            if (c == 'a') {
                v.push_back(1);
            } else if (c == 'b') {
                if (v.empty() || v.back() != 1) return false;
                v.back() = 2;
            } else {
                if (v.empty() || v.back() != 2) return false;
                v.pop_back();
            }
        }
        return v.empty();
    }
};