class ZeroEvenOdd {
private:
    int n;
    std::binary_semaphore sem_zero{1}, sem_even{0}, sem_odd{0};

public:
    ZeroEvenOdd(int n) {
        this->n = n;
    }

    // printNumber(x) outputs "x", where x is an integer.
    void zero(function<void(int)> printNumber) {
        for (int i = 1; i <= n; i++) {
            sem_zero.acquire();
            printNumber(0);
            if (i % 2 == 1) {
                sem_odd.release();
            } else {
                sem_even.release();
            }
        }
    }

    void even(function<void(int)> printNumber) {
        for (int i = 2; i <= n; i += 2) {
            sem_even.acquire();
            printNumber(i);
            sem_zero.release();
        }
    }

    void odd(function<void(int)> printNumber) {
        for (int i = 1; i <= n; i += 2) {
            sem_odd.acquire();
            printNumber(i);
            sem_zero.release();
        }
    }
};