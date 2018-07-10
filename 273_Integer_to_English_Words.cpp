class Solution {
    const vector<string> A = { "", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten",
                          "Eleven", "Twelve", "Thirteen", "Fourteen", "Fifteen", "Sixteen", "Seventeen", "Eighteen", "Nineteen" };
    const vector<string> B = { "", "Ten", "Twenty", "Thirty", "Forty", "Fifty", "Sixty", "Seventy", "Eighty", "Ninety" };
    const vector<string> C = { "", "Thousand", "Million", "Billion" };

    string f(int n) {
        if (n < 20) return A[n];
        if (n < 100) return B[n / 10] + (n % 10 ? " " : "") + A[n % 10];
        return A[n / 100] + " Hundred" + (n % 100 ? " " : "") + f(n % 100);
    }
public:
    string numberToWords(int num) {
        if (num == 0) return "Zero";
        int i = 0;
        string r = "";
        while (num > 0) {
            if (num % 1000)
                r = f(num % 1000) + (i > 0 ? " " + C[i]: "") + (r.empty() ? "" : " ") + r;
            num /= 1000;
            i++;
        }
        return r;
    }
};