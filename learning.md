rust atomics and locks ch1

multiple threads can run concurrently within the same program and can be spawned anytime
main thread ends, entire program ends
ARC: Atomically referenced counter, can be used to provide shared ownership (to make sure data lives) as long as one thread is  using it
&T is a shared reference, &mut t is a mutable shared reference. Shared references don't allow mutability while the latter does
Cell and RefCell (single threaded) are implementations of interior mutability. RwLock, Mutex and Atomics are all multi threaded equivalents
Cell and atomics allow replacing the value as a whole, while RefCell, Mutex and RwLock allows you to directly mutate the shared value. It does this by dynamically enforcing access rules
You can use thread parking to wait for some condition. Using this alone can cause issues as you cannot assume a thread is waited when called.
You can use Convar when using mutex, as convar provides a wait and unwait condition for data protected by mutex.

rust atomics and locks ch2
atomics are indivisible instructions, fully completed or havent completed yet.
atomic instructions contain a variety of low level instructions for managing shared state across threads. Some instrucitons are not available on some machines.
Atomic instrucitons don't always return the value after updating, posing conditions for race.
If you're setting id's to threads using atomic variables to keep track of the amount of id's, you can pose yourself into a situation where there are too many id's. This could either panic or do an overflow wrap. On specific machines, this can be a problem.
If you can use compare exchange to solve the problem above, checking and acting on the id. Compare exchange weak is most typically used, and faster on some platforms.