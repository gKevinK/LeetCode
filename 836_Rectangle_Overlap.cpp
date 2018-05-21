class Solution {
public:
    bool isRectangleOverlap(vector<int>& rec1, vector<int>& rec2) {
        return !( max(rec1[0], rec1[2]) <= min(rec2[0], rec2[2]) || min(rec1[0], rec1[2]) >= max(rec2[0], rec2[2]) )
            && !( max(rec1[1], rec1[3]) <= min(rec2[1], rec2[3]) || min(rec1[1], rec1[3]) >= max(rec2[1], rec2[3]) );
    }
};