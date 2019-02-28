class Solution {
public:
    vector<int> addToArrayForm(vector<int>& A, int K) {
        vector<int> res;
        string sk = to_string(K);
        int i = A.size() - 1, j = sk.size() - 1, carry = 0;
        for (; i >= 0 || j >= 0 || carry; i--, j--) {
            if (i >= 0) carry += A[i];
            if (j >= 0) carry += sk[j] - '0';
            res.push_back(carry % 10);
            carry /= 10;
        }
        return vector<int>(res.rbegin(), res.rend());
    }
};