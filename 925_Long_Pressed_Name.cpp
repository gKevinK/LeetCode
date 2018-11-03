class Solution {
public:
    bool isLongPressedName(string name, string typed) {
        int s1 = name.size(), s2 = typed.size(), i = 0, j = 0;
        while (i < s1 && j < s2) {
            if (name[i] != typed[j]) return false;
            int n1 = 1, n2 = 1;
            while (i + n1 < s1 && name[i] == name[i + n1]) n1++;
            while (j + n2 < s2 && typed[j] == typed[j + n2]) n2++;
            if (n1 > n2) return false;
            i += n1;
            j += n2;
        }
        return i == s1 && j == s2;
    }
};