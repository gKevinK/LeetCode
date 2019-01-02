class Solution {
    unordered_map<string, string> m;
    
    inline char i2c(int r) {
        return r < 10 ? ('0' + r) : (r < 36 ? ('a' + r - 10) : ('A' + r - 36));
    }
public:

    // Encodes a URL to a shortened URL.
    string encode(string longUrl) {
        string key = "";
        for (int i = 0; i < 6; i++)
            key += i2c(rand() % 62);
        while (m.find(key) != m.end())
            key += i2c(rand() % 62);
        m[key] = longUrl;
        return key;
    }

    // Decodes a shortened URL to its original URL.
    string decode(string shortUrl) {
        return m[shortUrl];
    }
};

// Your Solution object will be instantiated and called as such:
// Solution solution;
// solution.decode(solution.encode(url));