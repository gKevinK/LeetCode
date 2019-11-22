class DJS {
public:
    vector<int> parent;
    vector<int> size;
    vector<int> rank;
    
    DJS(int s) {
        parent = vector<int>(s, s - 1);
        size = vector<int>(s, 0);
        rank = vector<int>(s, 1);
    }
    
    int Find(int a) {
        while (parent[a] != a) a = parent[a];
        return a;
    }
    
    void Insert(int a) {
        parent[a] = a;
        size[a] = 1;
    }
    
    void Merge(int a, int b) {
        int xa = Find(a), xb = Find(b);
        if (xa == xb) return;
        if (rank[xa] < rank[xb]) swap(xa, xb);
        parent[xb] = xa;
        size[xa] += size[xb];
        rank[xa] = max(rank[xa], rank[xb] + 1);
    }
};

class Solution {
    int M, N;
    int id(int x, int y) {
        return x * N + y;
    }
public:
    vector<int> hitBricks(vector<vector<int>>& grid, vector<vector<int>>& hits) {
        M = grid.size();
        N = grid[0].size();
        auto g = grid;
        for (auto & h : hits)
            g[h[0]][h[1]] = 0;
        DJS x(M * N + 1);
        for (int i = 0; i < M; ++i) for (int j = 0; j < N; ++j) {
            if (g[i][j] == 0) continue;
            x.Insert(id(i, j));
            if (i == 0) x.Merge(M * N, id(i, j));
            if (i > 0 && g[i - 1][j]) x.Merge(id(i - 1, j), id(i, j));
            if (j > 0 && g[i][j - 1]) x.Merge(id(i, j - 1), id(i, j));
        }
        vector<int> res(hits.size());
        int last = x.size[x.Find(M * N)];
        for (int k = hits.size() - 1; k >= 0; --k) {
            int i = hits[k][0], j = hits[k][1];
            if (grid[i][j] == 0) continue;
            g[i][j] = 1;
            x.Insert(id(i, j));
            if (i == 0) x.Merge(M * N, id(i, j));
            if (i > 0 && g[i - 1][j]) x.Merge(id(i - 1, j), id(i, j));
            if (i+1<M && g[i + 1][j]) x.Merge(id(i + 1, j), id(i, j));
            if (j > 0 && g[i][j - 1]) x.Merge(id(i, j - 1), id(i, j));
            if (j+1<N && g[i][j + 1]) x.Merge(id(i, j + 1), id(i, j));
            res[k] = max(0, x.size[x.Find(M * N)] - last - 1);
            last = x.size[x.Find(M * N)];
        }
        return res;
    }
};