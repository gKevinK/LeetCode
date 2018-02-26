class Solution {
public:
    int compareVersion(string version1, string version2) {
        int p1 = 0, p2 = 0;
        while (p1 < version1.size() || p2 < version2.size()) {
            int i1 = 0, i2 = 0;
            while (p1 < version1.size() && version1[p1] != '.')
                i1 = i1 * 10 + version1[p1++] - '0';
            while (p2 < version2.size() && version2[p2] != '.')
                i2 = i2 * 10 + version2[p2++] - '0';
            if (i1 < i2) return -1;
            else if (i1 > i2) return 1;
            p1++; p2++;
        }
        return 0;
    }
};