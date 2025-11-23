class Solution {
public:
    vector<string> fullJustify(vector<string>& words, int maxWidth) {
        vector<string> result;
        for (size_t i = 0, j; i < words.size(); i = j) {
            int gap_num = 0, char_num = words[i].size();
            j = i + 1;
            while (j < words.size()) {
                if (char_num + words[j].size() + gap_num + 1 > maxWidth)
                    break;
                char_num += words[j].size();
                gap_num++;
                j++;
            }
            string line = words[i];
            line.reserve(maxWidth);
            if (j == words.size()) {
                for (size_t k = i + 1; k < j; k++) {
                    line.push_back(' ');
                    line.append(words[k]);
                }
                while (line.size() < maxWidth)
                    line.push_back(' ');
            } else {
                int space_char = maxWidth - char_num;
                for (size_t k = i + 1; k < j; k++) {
                    int this_gap = (space_char + gap_num - 1) / gap_num;
                    space_char -= this_gap;
                    gap_num -= 1;
                    for (int l = 0; l < this_gap; l++)
                        line.push_back(' ');
                    line.append(words[k]);
                }
                while (line.size() < maxWidth)
                    line.push_back(' ');
            }
            result.push_back(line);
        }
        return result;
    }
};