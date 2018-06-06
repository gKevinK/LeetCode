/*
// Employee info
class Employee {
public:
    // It's the unique ID of each node.
    // unique id of this employee
    int id;
    // the importance value of this employee
    int importance;
    // the id of direct subordinates
    vector<int> subordinates;
};
*/
class Solution {
public:
    int getImportance(vector<Employee*> employees, int id) {
        unordered_map<int, Employee*> m;
        for (auto & pe : employees) {
            m[pe->id] = pe;
        }
        int s = 0;
        queue<int> q;
        q.push(id);
        while (!q.empty()) {
            int id2 = q.front();
            q.pop();
            s += m[id2]->importance;
            for (int & id3 : m[id2]->subordinates)
                q.push(id3);
        }
        return s;
    }
};