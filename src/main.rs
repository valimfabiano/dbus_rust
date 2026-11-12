use std::collections::HashMap;
use zbus::{connection, fdo, interface, message::Header, Connection};
use zbus_polkit::policykit1::{AuthorityProxy, CheckAuthorizationFlags, Subject};

struct MaintenanceManager {
    is_active: bool,
}

impl MaintenanceManager {
    pub fn new() -> Self {
        println!("[Daemon] Initializing default state (maintenance = false)...");
        Self { is_active: false }
    }
}

#[interface(name = "com.example.SystemMaintenance")]
impl MaintenanceManager {
    /// Read-only method: Any user can query the current state.
    async fn state(&self) -> bool {
        println!("[Daemon] State query received.");
        self.is_active
    }

    /// Mutating method: Protected by Polkit
    async fn set_state(
        &mut self,
        new_state: bool,
        #[zbus(header)] header: Header<'_>,
        #[zbus(connection)] conn: &Connection,
    ) -> fdo::Result<String> {
        println!("[Daemon] SetState request received. Verifying Polkit authorization...");

        // 1. Identify the sender's D-Bus unique name for logging
        let sender = header
            .sender()
            .ok_or_else(|| fdo::Error::Failed("Failed to identify sender".into()))?;

        // 2. zbus 6: Create a Polkit Subject directly from the D-Bus message header!
        let subject = Subject::new_for_message_header(&header)
            .map_err(|e| fdo::Error::Failed(format!("Failed to create Subject: {}", e)))?;

        // 3. Connect to the Polkit daemon using the AuthorityProxy
        let polkit_proxy = AuthorityProxy::builder(conn)
            .build()
            .await
            .map_err(|e| fdo::Error::Failed(format!("Polkit proxy error: {}", e)))?;

        // 4. Ask Polkit if this sender is authorized
        let check_result = polkit_proxy
            .check_authorization(
                &subject,
                "com.example.systemmaintenance.set-state",
                &HashMap::new(),
                CheckAuthorizationFlags::ALLOW_USER_INTERACTION.into(),
                "",
            )
            .await
            .map_err(|e| fdo::Error::Failed(format!("Auth check failed: {}", e)))?;

        // 5. Evaluate the result
        if !check_result.is_authorized {
            println!("[Daemon] WARNING: Authorization denied for sender {}", sender);
            return Err(fdo::Error::AccessDenied("Polkit authorization failed".into()));
        }

        // 6. Execute the change if authorized
        println!("[Daemon] Authorization GRANTED. Changing state to {}", new_state);
        self.is_active = new_state;

        Ok(format!("Success: System maintenance mode is now {}", self.is_active))
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

    println!("[Daemon] Successfully connected to the System Bus with Polkit security.");
    println!("[Daemon] Waiting for D-Bus commands...");

    std::future::pending::<()>().await;

    Ok(())
}