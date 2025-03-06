class Solution {
public:
    int scheduleCourse(vector<vector<int>>& courses) {
        priority_queue<int> pq;
        int days = 0, r = 0;
        sort(courses.begin(), courses.end(), [](auto & a, auto & b) {
            return a[1] < b[1];
        });
        for (auto & c : courses) {
            if (days + c[0] <= c[1]) {
                r++;
                days += c[0];
                pq.push(c[0]);
            } else if (!pq.empty() && pq.top() > c[0]) {
                days -= pq.top();
                pq.pop();
                days += c[0];
                pq.push(c[0]);
            }
        }
        return r;
    }
};