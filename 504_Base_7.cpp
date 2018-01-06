class Solution {
public:
    string convertToBase7(int num) {
        if (num < 0) return "-" + convertToBase7(-num);
        if (num == 0) return "0";
        string s;
        while (num) {
            s = to_string(num % 7) + s;
            num /= 7;
        }
        return s;
    }
};