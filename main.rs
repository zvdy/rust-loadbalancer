use std::sync::{Arc, Mutex};
use std::thread;

// Define the Server struct
struct Server {
    name: String,
    load: Mutex<usize>,
}

impl Server {
    // Constructor for Server
    fn new(name: String) -> Self {
        Server {
            name,
            load: Mutex::new(0),
        }
    }

    // Method to increase the load of the server
    fn increase_load(&self) {
        let mut load = self.load.lock().unwrap();
        *load += 1;
    }

    // Method to decrease the load of the server
    fn decrease_load(&self) {
        let mut load = self.load.lock().unwrap();
        *load -= 1;
    }

    // Method to get the current load of the server
    fn get_load(&self) -> usize {
        let load = self.load.lock().unwrap();
        *load
    }
}

// Define the LoadBalancer struct
struct LoadBalancer {
    servers: Vec<Arc<Server>>,
}

impl LoadBalancer {
    // Constructor for LoadBalancer
    fn new(servers: Vec<Arc<Server>>) -> Self {
        LoadBalancer { servers }
    }

    // Method to get the next server with the least load
    fn next_server(&self) -> Arc<Server> {
        self.servers
            .iter()
            .min_by_key(|server| *server.load.lock().unwrap())
            .unwrap()
            .clone()
    }

    // Method to increase the load of a specific server
    fn load_server(&self, server_name: &str) {
        if let Some(server) = self.servers.iter().find(|s| s.name == server_name) {
            server.increase_load();
        }
    }
}

fn main() {
    // Create a list of servers
    let servers = vec![
        Arc::new(Server::new("server1".to_string())),
        Arc::new(Server::new("server2".to_string())),
    ];
    // Create a load balancer with the list of servers
    let load_balancer = Arc::new(LoadBalancer::new(servers));

    // Increase the load of server1
    load_balancer.load_server("server1");

    // Create a list of thread handles
    let mut handles = vec![];
    for _ in 0..10 {
        let load_balancer = Arc::clone(&load_balancer);
        // Spawn a new thread for each request
        let handle = thread::spawn(move || {
            // Get the next server with the least load
            let server = load_balancer.next_server();
            // Increase the load of the server
            server.increase_load();
            // Print the server name and its current load
            println!("Request served by {}. Current load: {}", server.name, server.get_load());
            // Decrease the load of the server
            server.decrease_load();
        });
        // Add the thread handle to the list
        handles.push(handle);
    }

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }
}
