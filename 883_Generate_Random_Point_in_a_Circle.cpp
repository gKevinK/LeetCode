class Solution {
    double r, x, y;
public:
    Solution(double radius, double x_center, double y_center) {
        r = radius;
        x = x_center;
        y = y_center;
    }
    
    vector<double> randPoint() {
        double l = r * sqrt((double)rand() / (double)RAND_MAX);
        double d = 2 * 3.14159265359 * ((double)rand() / (double)RAND_MAX);
        return { x + l * cos(d), y + l * sin(d) };
    }
};

/**
 * Your Solution object will be instantiated and called as such:
 * Solution obj = new Solution(radius, x_center, y_center);
 * vector<double> param_1 = obj.randPoint();
 */