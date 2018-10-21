class Solution {
public:
    vector<int> sortArrayByParityII(vector<int>& A) {
        vector<int> v(A.size());
        int i = 0, j = 0;
        for (int & n : A) {
            if (n % 2)
                v[(j++) * 2 + 1] = n;
            else
                v[(i++) * 2] = n;
        }
        return v;
    }
};