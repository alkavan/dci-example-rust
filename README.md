# Rust DCI Example

## Data

The data remains "_what the system is_."  
The data part of the DCI architecture is its (relatively) static data model with relations.

## Context

The context is the class (or its instance) whose code includes the Roles for a given algorithm, scenario, or use case,
as well as the code to map these Roles into objects at run time and to enact the use case.

## Interaction

The interaction is "_what the system does_."  
The interaction is implemented as Roles which are played by objects at run time.  
These objects combine the state and methods of a data (domain)
object with methods (but no state, as Roles are stateless) from one or more Roles.

## This Example

### Running

To see the log output of the program set
`RUST_LOG=info` ([enabling logging](https://docs.rs/env_logger/latest/env_logger/#enabling-logging))
in your machine environment.

### Output

```
 INFO  dci_example_rust::context > balance of account#420: 1000000.000000
 INFO  dci_example_rust::context > balance of account#720: 1000000.000000
 INFO  dci_example_rust::context > transferred 150000.000000 from account#420 to account#720
 INFO  dci_example_rust::context > balance of account#420: 850000.000000
 INFO  dci_example_rust::context > balance of account#720: 1150000.000000
```