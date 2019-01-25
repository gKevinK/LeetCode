class Solution {
public:
    vector<int> powerfulIntegers(int x, int y, int bound) {
        unordered_set<int> s;
        vector<int> px = { 1 }, py = { 1 };
        if (x != 1) while (px.back() * x < bound)
            px.push_back(px.back() * x);
        if (y != 1) while (py.back() * y < bound)
            py.push_back(py.back() * y);
        for (int i = 0; i < px.size(); i++)
            for (int j = 0; j < py.size() && px[i] + py[j] <= bound; j++)
                s.insert(px[i] + py[j]);
        return vector<int>(s.begin(), s.end());
    }
};