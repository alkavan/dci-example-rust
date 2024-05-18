# Rust DCI Example

DCI (Data, Context, Interaction) is an object-oriented design paradigm
that aims to improve readability and understandability of code.

Objects (**Data**) within a particular scenario (**Context**),  
collaborate by playing specific roles (**Interaction**),  
separating what _the system is_ (the domain) from what _the system does_ (use-cases and scenarios).

Maintaining the interaction's dynamic behavior and the data object's static properties separately,  
DCI aims to clarify and simplify the system structure, making it more intuitive and human-friendly.

## Data

Data represents the objects in your system or _"what the system is"_.  
These objects are static and don't change over time, they also contain relations.

## Context

A context represents a scenario or use-case within which the data objects interact.  
Each context corresponds to a system operation and is triggered by a specific event.  
It also has code to mapping roles into objects at run time and to enact the use-case or scenario.

## Interaction

Interaction is implemented as _roles_ are stateless and are _played_ by _objects_ at run time.  
These objects **combine** the _state_ and _methods_ of a _data_ (domain).  
It represents _"what the system does"_.

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
