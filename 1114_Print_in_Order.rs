class Foo {
    int rank = 1;
    std::mutex mutex;
    std::condition_variable var;
public:
    Foo() {
        
    }

    void first(function<void()> printFirst) {
        std::unique_lock<std::mutex> ulock(mutex);
        var.wait(ulock, [&] { return rank == 1; });
        // printFirst() outputs "first". Do not change or remove this line.
        printFirst();
        rank = 2;
        var.notify_all();
    }

    void second(function<void()> printSecond) {
        std::unique_lock<std::mutex> ulock(mutex);
        var.wait(ulock, [&] { return rank == 2; });
        // printSecond() outputs "second". Do not change or remove this line.
        printSecond();
        rank = 3;
        var.notify_all();
    }

    void third(function<void()> printThird) {
        std::unique_lock<std::mutex> ulock(mutex);
        var.wait(ulock, [&] { return rank == 3; });
        // printThird() outputs "third". Do not change or remove this line.
        printThird();
        rank = 1;
        var.notify_all();
    }
};