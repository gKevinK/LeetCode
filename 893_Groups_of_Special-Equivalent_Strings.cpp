class Solution {
public:
    int numSpecialEquivGroups(vector<string>& A) {
        unordered_set<string> s;
        for (string & a : A) {
            string a1, a2;
            for (int i = 0; i < a.size(); i++)
                (i % 2 ? a1 : a2).push_back(a[i]);
            sort(a1.begin(), a1.end());
            sort(a2.begin(), a2.end());
            s.insert(a1 + a2);
        }
        return s.size();
    }
};