class Solution {
public:
    bool checkPerfectNumber(int num) {
        if (num <= 0)
            return false;
        if (num <= 3)
            return false;
        int sum = 1, sqr = sqrt(num);
        if (sqr * sqr == num)
            sum -= sqr;
        for (int i = sqr; i > 1; i--) {
            if (num % i == 0)
                sum += i + num / i;
            if (sum > num)
                return false;
        }
        return sum == num;
    }
};