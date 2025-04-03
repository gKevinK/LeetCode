class Solution {
public:
    bool eq(string a, string b, int i) {
        for (int j = 0; j < b.size(); j++) {
            if (a[j + i] != b[j])
                return false;
        }
        return true;
    }
    int strStr(string haystack, string needle) {
        int a = haystack.size();
        int b = needle.size();
        for (int i = 0; i <= a - b; i++) {
            if (eq(haystack, needle, i))
                return i;
        }
        return -1;
    }
};