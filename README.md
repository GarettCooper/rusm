# RUst Service Modules (rusm)

rusm is an opinionated paradigm for building service-like rust crates which can be composed together for many
of the benefits of Service-oriented Architecture without the significant overhead of separately deployable microservices.
rusm lets you build a bundle of modules which can be deployed together in a monolith at first and then pulled apart into
individually deployed units if/as it becomes necessary without modifying any service code, letting you move fast while
remaining prepared for the future.
