rust atomics and locks ch1

multiple threads can run concurrently within the same program and can be spawned anytime
main thread ends, entire program ends
ARC: Atomically referenced counter, can be used to provide shared ownership (to make sure data lives) as long as one thread is  using it
&T is a shared reference, &mut t is a mutable shared reference. Shared references don't allow mutability while the latter does
Cell and RefCell (single threaded) are implementations of interior mutability. RwLock, Mutex and Atomics are all multi threaded equivalents
Cell and atomics allow replacing the value as a whole, while RefCell, Mutex and RwLock allows you to directly mutate the shared value. It does this by dynamically enforcing access rules
You can use thread parking to wait for some condition. Using this alone can cause issues as you cannot assume a thread is waited when called.
You can use Convar when using mutex, as convar provides a wait and unwait condition for data protected by mutex.
