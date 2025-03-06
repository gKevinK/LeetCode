class Solution {
public:
    bool canThreePartsEqualSum(vector<int>& A) {
        long sum = 0, s = 0;
        int seg = 0;
        for (int & i : A)
            sum += i;
        if (sum % 3)
            return false;
        s = sum / 3;
        sum = 0;
        for (int & i : A) {
            sum += i;
            if (sum == s) {
                if (seg == 0) {
                    seg++;
                    s *= 2;
                } else
                    return true;
            }
        }
        return false;
    }
};