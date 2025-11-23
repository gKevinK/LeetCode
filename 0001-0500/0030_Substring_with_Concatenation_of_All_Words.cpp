class Solution {
public:
    vector<int> findSubstring(string s, vector<string>& words) {
        size_t word_len = words[0].size();
        size_t total_len = word_len * words.size();
        if (s.size() < total_len)
            return {};
        unordered_map<string, int> count;
        for (auto & word : words)
            count[word]++;

        vector<int> result;
        unordered_map<string, int> visit;
        for (size_t offset = 0; offset < word_len; offset++) {
            size_t left = offset;
            for (size_t right = offset; right <= s.size(); right += word_len) {
                string sub = s.substr(right, word_len);
                if (count.find(sub) == count.end()) {
                    if (left + total_len <= right) {
                        visit.clear();
                        size_t l = left;
                        for (size_t r = l; r < right; r += word_len) {
                            string sub_r = s.substr(r, word_len);
                            visit[sub_r]++;

                            while (visit[sub_r] > count[sub_r]) {
                                string sub_l = s.substr(l, word_len);
                                visit[sub_l]--;
                                l += word_len;
                            }

                            if (l + total_len == r + word_len)
                                result.push_back(l);
                        }
                    }
                    left = right + word_len;
                }
            }
        }
        return result;
    }
};