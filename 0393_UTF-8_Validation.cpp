class Solution {
public:
    bool validUtf8(vector<int>& data) {
        int p = 0;
        while (p < data.size()) {
            if (data[p] <= 0x7F) p++;
            else if (0xC0 <= data[p] && data[p] <= 0xDF) {
                if (!(0x80 <= data[p+1] && data[p+1] <= 0xBF)) return false;
                p += 2;
            } else if (0xE0 <= data[p] && data[p] <= 0xEF) {
                if (!(0x80 <= data[p+1] && data[p+1] <= 0xBF
                      && 0x80 <= data[p+2] && data[p+2] <= 0xBF)) return false;
                p += 3;
            } else if (0xF0 <= data[p] && data[p] <= 0xF7) {
                if (!(0x80 <= data[p+1] && data[p+1] <= 0xBF
                      && 0x80 <= data[p+2] && data[p+2] <= 0xBF
                      && 0x80 <= data[p+3] && data[p+3] <= 0xBF)) return false;
                p += 4;
            } else return false;
        }
        return true;
    }
};