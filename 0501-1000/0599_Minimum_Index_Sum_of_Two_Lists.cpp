class Solution {
public:
    vector<string> findRestaurant(vector<string>& list1, vector<string>& list2) {
        if (list1.size() > list2.size())
            return findRestaurant(list2, list1);
        vector<string> res;
        int s = INT_MAX;
        unordered_map<string, int> m;
        for (int i = 0; i < list1.size(); i++)
            m[list1[i]] = i;
        for (int i = 0; i < list2.size(); i++) {
            if (m.find(list2[i]) != m.end() && i + m[list2[i]] <= s) {
                if (i + m[list2[i]] < s) {
                    res.clear();
                    s = i + m[list2[i]];
                }
                res.push_back(list2[i]);
            }
        }
        return res;
    }
};