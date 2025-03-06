class Solution {
public:
    int distributeCandies(vector<int>& candies) {
        unordered_set<int> s(candies.begin(), candies.end());
        return min(candies.size() / 2, s.size());
        // vector<bool> v(200001, 0);
        // for (int & i : candies)
        //     v[i + 100000] = true;
        // int r = 0;
        // for (auto i : v)
        //     r += i;
        // return min(r, candies.size() / 2);
    }
};