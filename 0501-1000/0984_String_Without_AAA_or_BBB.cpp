class Solution {
public:
    string strWithout3a3b(int A, int B) {
        string s;
        int ra = 0, rb = 0;
        while (A || B) {
            if ((A >= B && ra < 2) || (A < B && rb == 2)) {
                s += 'a';
                A--;
                ra++;
                rb = 0;
            } else {
                s += 'b';
                B--;
                ra = 0;
                rb++;
            }
        }
        return s;
    }
};