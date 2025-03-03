class StockSpanner {
    vector<pair<int, int>> stack;
    int pos = 0;
public:
    StockSpanner() {
        
    }
    
    int next(int price) {
        while (!stack.empty() && stack.back().first <= price)
            stack.pop_back();
        int r = pos - (stack.empty() ? -1 : stack.back().second);
        stack.push_back({ price, pos++ });
        return r;
    }
};

/**
 * Your StockSpanner object will be instantiated and called as such:
 * StockSpanner obj = new StockSpanner();
 * int param_1 = obj.next(price);
 */