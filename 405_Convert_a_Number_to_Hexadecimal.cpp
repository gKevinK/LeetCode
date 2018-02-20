class Solution {
public:
    string toHex(int num) {
        string s;
        if (num == 0) return "0";
        unsigned int n = *(reinterpret_cast<unsigned int *>(&num));
        while (n != 0) {
            int i = n & 0xF;
            s.push_back(i >= 10 ? i - 10 + 'a' : i + '0');
            n >>= 4;
        }
        for (int i = 0, j = s.size() - 1; i < j; i++, j--)
            swap(s[i], s[j]);
        return s;
    }
};