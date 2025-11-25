#include <barrier>

class H2O {
    std::barrier<> bar{3};
    std::counting_semaphore<> sem_h{2}, sem_o{1};
public:
    H2O() {
        
    }

    void hydrogen(function<void()> releaseHydrogen) {
        sem_h.acquire();
        bar.arrive_and_wait();
        // releaseHydrogen() outputs "H". Do not change or remove this line.
        releaseHydrogen();
        sem_h.release();
    }

    void oxygen(function<void()> releaseOxygen) {
        sem_o.acquire();
        bar.arrive_and_wait();
        // releaseOxygen() outputs "O". Do not change or remove this line.
        releaseOxygen();
        sem_o.release();
    }
};