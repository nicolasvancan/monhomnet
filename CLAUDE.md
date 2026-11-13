# monhumnet

Rust project (edition 2024) tha will implement a inovative approach on neural networks. The core idea in this project is to be able to train and use the new neural network here created to train and evaluate different sorts of data.

What does monhumnet stand for?
- mon: Montanha (My last name)
- hum: Human based
- net: network

This software is not reinventing the wheel nor recreating something similar to pytorch, instead it is trying to approach the neural network problems differently.

## Domain concepts

Neural networks and all the diferent usages are present in almost everuthing that we do daily. One concern that I have is that, although it is extremly good in generating text, recognizing patterns, computer vision, it is still not energy efficient.

The training process and weights calculation uses a lot of energy and hardware, creating big models that require also big infra to run.

Why not try to change how this process happen and use another approach? This project aims to mimic some other human brain behaviours regarding neuroplasticity and learning capabilities.

Instead of calculating weights of different networks layers, why not simply choose where should a neuron create another connection?

The differences here are that the neurons are immutable in terms of weights, what, at first, changes, are the connections and their importance to a prediction.

### Deeper concepts and theoretical ideas

To find more about what I am thinking for the project, all files are located under **./concept**:

- **idea.md**: (Still not exist) - The main ideation, the chore concepts and the overall idea to the implementation
- **neuron.md**: (Still not exist) - Describe what is the neuron and how it should work 
- **synapses.md**: (Still not exist) - Describe what are the synapses (connections between neurons) and how their purpose

## Commands

- Build: `cargo build`
- Run: `cargo run`
- Test: `cargo test`
- Lint: `cargo clippy -- -D warnings`
- Format: `cargo fmt`

## Coding preferences

- Prefer simple, direct code over abstraction. Write the straightforward version first.
- Don't add traits, generics, or new modules until there are at least two concrete uses for them.
- Prefer plain functions and structs; avoid design patterns (builders, factories, etc.) unless asked.
- Keep functions short and focused; a little duplication is better than the wrong abstraction.
- Don't add dependencies without asking first.
- Only change what the task requires; no unrelated refactors.
- Always ask when more than one implementation is valid
- Write simple and direct comments, unless required to write long ones

## Testing

- Write unit tests in a `#[cfg(test)] mod tests` block at the bottom of the same file.
- Test behavior, not implementation details.
- Keep tests simple and readable; one behavior per test, descriptive names (`fn rejects_empty_input()`).
- No mocking frameworks or test-only abstractions; prefer real values.
- Run `cargo test` before saying a task is done.



