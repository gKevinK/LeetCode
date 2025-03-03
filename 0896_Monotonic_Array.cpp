class Solution {
public:
    bool isMonotonic(vector<int>& A) {
        int d = 0;
        for (int i = 1; i < A.size(); i++) {
            int diff = A[i] - A[i - 1];
            if (diff * d < 0)
                return false;
            if (diff != 0 && d == 0)
                d = diff > 0 ? 1 : -1;
        }
        return true;
    }
};