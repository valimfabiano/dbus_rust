use zbus::{connection, proxy, Result};

/// This macro generates a proxy struct called `SystemMaintenanceProxy`.
/// It acts as a bridge, converting local async function calls into D-Bus messages.
#[proxy(
    interface = "com.example.SystemMaintenance",
    default_service = "com.example.SystemMaintenance",
    default_path = "/com/example/SystemMaintenance"
)]
trait SystemMaintenance {
    /// Mirrors the read-only method on the daemon.
    async fn state(&self) -> Result<bool>;

    /// Mirrors the mutating method on the daemon.
    async fn set_state(&self, new_state: bool) -> Result<String>;
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("[Client] Connecting to the System Bus...");
    
    // Connect to the System Bus (as a regular user)
    let conn = connection::Builder::system().build().await?;

    // Instantiate the proxy
    let proxy = SystemMaintenanceProxy::new(&conn).await?;

    // 1. Query the current state
    let current_state = proxy.state().await?;
    println!("[Client] The daemon reported state: {}", current_state);

    // 2. Attempt to toggle the state to the opposite of what it currently is
    let target_state = !current_state;
    println!("[Client] Requesting state change to: {}...", target_state);

    // 3. Handle the Polkit challenge
    // If Polkit requires a password, the D-Bus system will trigger the GUI prompt
    // while this future awaits.
    match proxy.set_state(target_state).await {
        Ok(success_msg) => {
            println!("[Client] Action Authorized! Daemon says: {}", success_msg);
        }
        Err(e) => {
            eprintln!("[Client] Action Denied! Error: {}", e);
        }
    }

    Ok(())
}