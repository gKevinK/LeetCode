class FooBar {
private:
    int n;
    
    std::mutex mutex;
    std::condition_variable var;
    bool flag = false;

public:
    FooBar(int n) {
        this->n = n;
    }

    void foo(function<void()> printFoo) {
        
        for (int i = 0; i < n; i++) {
            std::unique_lock<std::mutex> ulock(mutex);
            var.wait(ulock, [&] { return !flag; });
        	// printFoo() outputs "foo". Do not change or remove this line.
        	printFoo();
            flag = true;
            var.notify_one();
        }
    }

    void bar(function<void()> printBar) {
        
        for (int i = 0; i < n; i++) {
            std::unique_lock<std::mutex> ulock(mutex);
            var.wait(ulock, [&] { return flag; });
        	// printBar() outputs "bar". Do not change or remove this line.
        	printBar();
            flag = false;
            var.notify_one();
        }
    }
};