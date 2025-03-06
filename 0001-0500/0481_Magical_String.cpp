class Solution {
    string s = "122";
    int o = 2;
    
public:
    int magicalString(int n) {
        while (s.size() < n) {
            s += '2' - s.back() + '1';
            if (s[o++] == '2')
                s += s.back();
        }
        int c = 0;
        for (int i = 0; i < n; i++)
            if (s[i] == '1')
                c++;
        return c;
    }
};