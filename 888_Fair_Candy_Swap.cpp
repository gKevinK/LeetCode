class Solution {
public:
    vector<int> fairCandySwap(vector<int>& A, vector<int>& B) {
        int diff = 0;
        for (int & i : A) diff += i;
        for (int & i : B) diff -= i;
        diff /= 2;
        unordered_set<int> s(A.begin(), A.end());
        for (int & i : B)
            if (s.find(i + diff) != s.end())
                return { i + diff, i };
        return { 0, 0 };
    }
};