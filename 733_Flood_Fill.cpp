class Solution {
public:
    vector<vector<int>> floodFill(vector<vector<int>>& image, int sr, int sc, int newColor) {
        vector<pair<int, int>> d = { { -1, 0 }, { 0, -1 }, { 1, 0 }, { 0, 1 }};
        queue<pair<int, int>> q;
        int color = image[sr][sc];
        image[sr][sc] = newColor;
        q.push(make_pair(sr, sc));
        if (color == newColor)
            return image;
        while (!q.empty()) {
            int x = q.front().first, y = q.front().second;
            q.pop();
            for (auto & p : d) {
                int xt = x + p.first, yt = y + p.second;
                if (xt >= 0 && xt < image.size() && yt >= 0 && yt < image[0].size() && image[xt][yt] == color) {
                    image[xt][yt] = newColor;
                    q.push(make_pair(xt, yt));
                }
            }
        }
        return image;
    }
};