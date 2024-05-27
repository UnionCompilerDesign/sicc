// Step 1: get the resource pools in a variable.
let resource_pools: Arc<Mutex<ResourcePools>> = self.get_resource_pools();

// Step 2: try to lock the resource pools, expecting an error if it's already locked.
let resource_pools_lock: MutexGuard<ResourcePools> = resource_pools.try_lock().expect("Can't lock!");

// Step 3: do what you have to do with the resource pools through this lock.
resource_pools_lock.create_integer(FUNCTION ARGUMENTS);

// Step 4: drop the lock when you're done with modifying the resource pools so you don't cause locking issues
drop(resource_pools_lock);
