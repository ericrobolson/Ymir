A web (or platform agnostic) client app framework to build a faster, more responsive end user experience.

Tech idea:
* Elixir backend, Rust frontend + middle
* Elixir handles db's, connections, etc.
* Rust handles memory safety, perf critical sections, all logic, all client code

Key objectives to test
* Utilizes VM for device interaction
* * Input port (or whatever comes in to change the app state)
* * Output port (or whatever is displayed to the user, or leaves the app state such as the dom, network calls, etc.)
* Abstraction derived from implementation. 
* * Come up with a 'hardcoded' or struct-only variant. On the second similar implementation, add a comment saying these are similar. On the third make a trait. 
* * The intent being that the wrong abstraction is worse than no abstraction
* Experiments in stateful practices; using pass-by-move wherever possible to encourage a functional style
* Component driven design
* Metric based optimization
* Perfect research + optimization of state
* * This is a 'component' or 'intent' featured approach. Client state is stored in local or session storage, etc. 

Lighthouse Declaration:
* Component based design (ECS)
* Elm style messaging model
* WASM compatible




Reach Goals (Nice to have but not core effort)
* Privacy focused
* * Utilize encryption heavily. Client provides storage mechanism, meaning we don't need to deal with our own servers, instead clients entrust the data to their own stores?
* * Client owned storage solutions. This provides a way to absolve yourself of the issues related to clients and their data?

