class Solution {
    int get(int eggs, int drop) {
        static vector<vector<int>> cache;
        if (drop == 1) return 2;
        if (eggs == 1) return drop + 1;
        while (cache.size() <= drop) cache.push_back({});
        while (cache[drop].size() <= eggs) cache[drop].push_back(0);
        if (cache[drop][eggs] > 0) return cache[drop][eggs];
        int r = get(eggs - 1, drop - 1) + get(eggs, drop - 1);
        cache[drop][eggs] = r;
        // cout << drop << ' ' << eggs << ' ' << r << endl;
        return r;
    }
public:
    int superEggDrop(int K, int N) {
        for (int i = 1; i < 10000; i++)
            if (get(K, i) >= N + 1)
                return i;
        return -1;
    }
};