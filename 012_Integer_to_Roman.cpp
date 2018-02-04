class Solution {
public:
    string intToRoman(int num) {
        string a[] = {"", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX"};
        string b[] = {"", "X", "XX", "XXX", "XL", "L", "LX", "LXX", "LXXX", "XC"};
        string c[] = {"", "C", "CC", "CCC", "CD", "D", "DC", "DCC", "DCCC", "CM"};
        string d[] = {"", "M", "MM", "MMM"};
        return d[num/1000] + c[num/100%10]  + b[num/10%10] + a[num%10];
    }
};