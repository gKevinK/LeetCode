class Solution {
public:
    vector<pair<int, int>> getSkyline(vector<vector<int>>& buildings) {
        list<pair<int, int>> res;
        res.push_back({ 0, 0 });
        auto it = res.begin();
        for (auto & b : buildings) {
            while (it != res.end() && it->first < b[0]) it++;
            auto it2 = it;
            while (it2 != res.end() && it2->second > *******) it2++;
            
        }
        if (res.front().first == 0 && res.front().second == 0)
            res.pop_front();
        return vector<pair<int, int>>(res.begin(), res.end());
    }
};