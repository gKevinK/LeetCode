class Solution {
public:
    string largestTimeFromDigits(vector<int>& A) {
        sort(A.begin(), A.end());
        string r = "";
        do {
            if (A[0] >= 3) break;
            if (A[0] == 2 && A[1] >= 4) break;
            if (A[2] >= 6) continue;
            string t = "00:00";
            t[0] += A[0];
            t[1] += A[1];
            t[3] += A[2];
            t[4] += A[3];
            if (t > r) r = t;
        } while (next_permutation(A.begin(), A.end()));
        return r;
    }
};