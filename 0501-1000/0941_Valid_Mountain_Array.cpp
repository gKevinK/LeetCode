class Solution {
public:
    bool validMountainArray(vector<int>& A) {
        int state = 0;
        for (int i = 1; i < A.size(); ++i) {
            if (A[i - 1] == A[i])
                return false;
            if (state == 0) {
                if (A[i - 1] > A[i])
                    return false;
                else
                    state = 1;
            } else if (state == 1) {
                if (A[i - 1] > A[i])
                    state = 2;
            } else {
                if (A[i - 1] < A[i])
                    return false;
            }
        }
        return state == 2;
    }
};