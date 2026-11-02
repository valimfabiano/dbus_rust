use zbus::{connection, interface};

/// Represents the internal state of our system daemon.
struct MaintenanceManager {
    is_active: bool,
}

impl MaintenanceManager {
    /// Idiomatic constructor to initialize the default state.
    pub fn new() -> Self {
        println!("[Daemon] Initializing default state (maintenance = false)...");
        Self { is_active: false }
    }
}

/// The D-Bus interface definition.
/// This macro generates the necessary D-Bus introspection XML and handles routing.
#[interface(name = "com.example.SystemMaintenance")]
impl MaintenanceManager {
    /// Read-only method: Any user can query the current state without authentication.
    async fn state(&self) -> bool {
        println!("[Daemon] State query received. Current state: {}", self.is_active);
        self.is_active
    }

    /// Mutating method: Changes the system state.
    /// (In the next phase, Polkit will block this if the user is not an administrator).
    async fn set_state(&mut self, new_state: bool) -> String {
        println!("[Daemon] State change request received. New state: {}", new_state);
        
        // TODO: Security Phase! Polkit authorization check will be injected here.
        
        self.is_active = new_state;
        
        format!("Success: System maintenance mode is now {}", self.is_active)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manager = MaintenanceManager::new();

    let _conn = connection::Builder::system()
        .name("com.example.SystemMaintenance")
        .serve_at("/com/example/SystemMaintenance", manager)
        .build()
        .await?;

    println!("[Daemon] Successfully connected to the System Bus.");
    println!("[Daemon] Waiting for D-Bus commands...");

    // Keep the asynchronous runtime alive indefinitely to serve requests.
    std::future::pending::<()>().await;

    Ok(())
}