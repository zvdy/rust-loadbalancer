GitHub Copilot: # Load Balancer in Rust :crab:

This is a simple implementation of a load balancer in Rust. The load balancer distributes load across multiple servers.

## Server :computer:

The `Server` struct represents a server with a `name` and `load`. The `load` is protected by a `Mutex` to allow safe concurrent modifications.

- `new(name: String) -> Self`: Constructor for `Server`.
- `increase_load(&self)`: Increases the load of the server.
- `decrease_load(&self)`: Decreases the load of the server.
- `get_load(&self) -> usize`: Returns the current load of the server.

## Load Balancer :balance_scale:

The `LoadBalancer` struct represents a load balancer with a list of `servers`.

- `new(servers: Vec<Arc<Server>>) -> Self`: Constructor for `LoadBalancer`.
- `next_server(&self) -> Arc<Server>`: Returns the next server with the least load.
- `load_server(&self, server_name: &str)`: Increases the load of a specific server.

## Main Function :rocket:

The `main` function creates a list of servers and a load balancer. It then simulates requests by spawning threads that increase the load of the server, print the server name and its current load, and then decrease the load of the server.

## Running the Code :running:

To run the code, use the command `cargo run` in the terminal.

## Concurrency :zap:

This code uses Rust's concurrency features, such as `Arc` and `Mutex`, to safely share and modify data across multiple threads.

## License :page_with_curl:

This project is licensed under the [MIT License.](LICENSE)
