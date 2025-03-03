class Solution {
public:
    vector<int> asteroidCollision(vector<int>& asteroids) {
        vector<int> v;
        for (int i : asteroids) {
            while (!v.empty() && v.back() > 0 && i < 0) {
                if (v.back() == -i)
                    i = 0;
                else
                    i = v.back() > -i ? v.back() : i;
                v.pop_back();
            }
            if (i)
                v.push_back(i);
        }
        return v;
    }
};