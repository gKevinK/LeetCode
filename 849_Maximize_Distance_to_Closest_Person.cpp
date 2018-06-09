class Solution {
public:
    int maxDistToClosest(vector<int>& seats) {
        int m = 0;
    	for (int i = 0; i < seats.size(); i++) {
    		if (seats[i] == 1)
    			continue;
    		int j = i;
    		while (j < seats.size() && seats[j] == 0)
    			j++;
    		m = max((j == seats.size() || i == 0) ? j - i : (j - i + 1) / 2, m);
    		i = j;
    	}
        return m;
    }
};