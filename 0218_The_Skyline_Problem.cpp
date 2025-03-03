class Solution {
public:
    vector<pair<int, int>> getSkyline(vector<vector<int>>& buildings) {
        list<pair<int, int>> res;
        res.push_back({ 0, 0 });
        auto it = res.begin();
        for (auto & b : buildings) {
            while (next(it) != res.end() && next(it)->first < b[0]) it++;
            auto it2 = it;
            while (it2 != res.end() && it2->second > b[2]) it2++;
            if (it2 == res.end() || it2->first >= b[1]) continue;
            if (it2->second != b[2]) {
                if (it2->first < b[0]) res.insert(it2, *it2);
                res.insert(it2, { max(it2->first, b[0]), b[2] });
                if (it == it2 && it != res.begin()) it--;
            } else it2++;
            while (next(it2) != res.end() && next(it2)->first <= b[1]) {
                it2 = res.erase(it2);
            }
            it2->first = b[1];
        }
        if (res.front().first == 0 && res.front().second == 0)
            res.pop_front();
        return vector<pair<int, int>>(res.begin(), res.end());
    }
};