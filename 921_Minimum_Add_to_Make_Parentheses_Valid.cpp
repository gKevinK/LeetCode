class Solution {
public:
    int minAddToMakeValid(string S) {
        int num = 0, m = 0;
        for (char & c : S) {
            if (c == '(') {
                num++;
            } else {
                num--;
                m = min(m, num);
            }
        }
        return num - m - m;
    }
};