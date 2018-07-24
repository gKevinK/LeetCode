class Solution {
public:
    vector<pair<int, int>> reconstructQueue(vector<pair<int, int>>& people) {
        sort(people.begin(), people.end(), [](auto & a, auto & b) {
            return a.first == b.first ? a.second < b.second : a.first > b.first;
        });
        vector<pair<int, int>> v;
        for (auto & p : people) {
            // cout << p.first << " " << p.second << endl;
            v.insert(v.begin() + p.second, p);
        }
        return v;
    }
};