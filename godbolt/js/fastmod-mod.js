function fast() {
	return 122 % 31;
}

// Collect type information on next call of function
%PrepareFunctionForOptimization(fast)

// Call function once to fill type information
fast();

// Call function again to go from uninitialized -> pre-monomorphic -> monomorphic
fast();
%OptimizeFunctionOnNextCall(fast);
fast();
