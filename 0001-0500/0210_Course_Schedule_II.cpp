class Solution {
public:
    vector<int> findOrder(int numCourses, vector<pair<int, int>>& prerequisites) {
        vector<int> count(numCourses, 0);
        vector<vector<int>> pairs(numCourses, vector<int>());
        for (auto & p : prerequisites) {
            count[p.first]++;
            pairs[p.second].push_back(p.first);
        }
        vector<int> v1, v2, v3;
        for (int i = 0; i < numCourses; i++)
            if (count[i] == 0)
                v1.push_back(i);
        while (!v1.empty()) {
            for (int & i : v1) {
                for (int & j : pairs[i]) {
                    count[j]--;
                    if (count[j] == 0)
                        v2.push_back(j);
                }
                v3.push_back(i);
            }
            v1.clear();
            v1.swap(v2);
        }
        if (v3.size() == numCourses)
            return v3;
        return vector<int>();
    }
};