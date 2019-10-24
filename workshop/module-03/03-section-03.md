### Section III

Our last adaptive layer that needs to be constructed is the **broker** module that will facility the interaction between the library and the Kafka broker. 

Constructing the module will require us to make changes to the following files:

+ Cargo.toml (manifest)
+ src/lib.rs (library)
+ src/broker.rs (module)