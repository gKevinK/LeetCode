class Solution {
public:
    int subarrayBitwiseORs(vector<int>& A) {
        unordered_set<int> res, s1, s2;
        for (int & i : A) {
            s2 = { i };
            for (int j : s1) {
                s2.insert(i | j);
            }
            for (int j : s2) {
                res.insert(j);
            }
            s1.swap(s2);
        }
        return res.size();
    }
};