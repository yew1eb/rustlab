# A Parallel Mandelbrot Set Plotter 并行 Mandelbrot 集绘图仪

曼德博集合 https://zh.wikipedia.org/wiki/%E6%9B%BC%E
5%BE%B7%E5%8D%9A%E9%9B%86%E5%90%88
曼德博集合可以用复二次多项式来定义：

{\displaystyle f_{c}(z)=z^{2}+c\,}f_c(z) = z^2 + c \,
其中 {\displaystyle c}c 是一个复数参数。


This program plots the Mandelbrot set and writes it out as a PNG file. It uses Rust's concurrency primitives to distribute the work across eight threads.

这个程序绘制出曼德布洛特集，并将其作为 PNG 文件写出。它使用 Rust 的并发原语将工作分布到八个线程中。

Different commits show different implementation strategies:

不同的提交显示不同的实现策略:

## Branch single-threaded is the base version of the program. It does all the work on the main thread.
分支单线程是程序的基本版本，它在主线程上完成所有的工作。

## Branch bands splits the plotting area up into eight bands, and assigns one thread to each. This often makes poor use of the threads, because some bands take significantly longer than others to complete: once a fast thread completes its band, its CPU goes idle while its less fortunate brethren are still hard at work.
分支带将绘图区域分成八个带，并为每个带分配一个线程。这通常会导致线程的使用不足，因为有些带需要比其他带长得多的时间才能完成: 一旦一个快速线程完成了它的带，它的 CPU 就处于空闲状态，而它那些不那么幸运的兄弟们仍然在努力工作。

## Branch task-queue gets an almost perfect linear speedup from its threads. It splits the plotting area up into many more bands, and then has threads draw bands from a common pool until the pool is empty. When a thread finishes one band, it goes back for more work. Since the bands still take different amounts of time to render, the problem cited above still occurs, but on a much smaller scale.

分支任务队列从其线程获得了近乎完美的线性加速。它将绘图区域分割成更多的带状区域，然后让线程从一个公共池中绘制带状区域，直到该池为空。当一个线程完成一个波段时，它会返回进行更多的工作。由于渲染乐队仍然需要不同的时间，上面提到的问题仍然存在，只是规模要小得多。

## Branch lockfree uses Rust's atomic types to implement a lock-free iterator type, and uses that to dole out bands from the pool instead of a mutex-protected count. On Linux, this is no faster than the mutex-based version, which isn't too surprising: on Linux, locking and unlocking an uncontended mutex is simply a pair of atomic operations.

分支 lockfree 使用 Rust 的原子类型来实现一个无锁迭代器类型，并使用它从池中分配带数，而不是互斥体保护的计数。在 Linux 上，这并不比基于互斥对象的版本快，这并不太令人惊讶: 在 Linux 上，锁定和解锁一个无争用互斥对象只是一对原子操作。

## Branch rayon uses the Rayon library instead of Crossbeam. Rayon provides a parallel iterator API that makes our code much simpler. It looks a lot like Rust code that uses plain old iterators.

人造丝分支使用人造丝图书馆而不是 Crossbeam。Rayon 提供了一个并行迭代器 API，使我们的代码更加简单。它看起来很像 Rust 代码，使用普通的老迭代器。
