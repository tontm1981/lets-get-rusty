### Static Dispatch

Is when the compiler knows which concrete methods to call at compile time. All generics are switched to concrete type at compile time.


### Dynamic dispatch

Is when the compiler can't know which concrete methods to call at compile time. In this case, we must use trait objects (`Box`ed trait)