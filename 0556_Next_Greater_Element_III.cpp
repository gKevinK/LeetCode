class Solution {
public:
    int nextGreaterElement(int n) {
        string s = to_string(n);
        if (!next_permutation(s.begin(), s.end())) return -1;
        auto r1 = stol(s);
        return r1 > INT_MAX ? -1 : r1;
    }
};