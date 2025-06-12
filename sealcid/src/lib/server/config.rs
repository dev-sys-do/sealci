use crate::common::proto::{
    AgentMutation, ControllerMutation, MonitorMutation, ReleaseAgentMutation, SchedulerMutation,
};

pub trait Update<Mutation> {
    /// Updates the configuration with the given mutation.
    fn update(&mut self, mutation: Mutation);
}

#[derive(Debug, Clone)]
pub struct GlobalConfig {
    // Example: 8080
    pub monitor_port: String, // default: "9001"
    // Example: http://hugo.fr
    pub controller_host: String, // default: "http://localhost"
    // Example: 8080
    pub controller_port: String, // default: "8080"
    // Postgres url for the controller
    pub database_url: String, // default: "postgres://user:password@localhost/db"

    // Example: http://hugo.fr
    pub release_agent_host: String, // default: "http://localhost"
    // Example: 8080
    pub release_agent_port: String, // default: "8080"

    // Other configuration for the release agent
    pub passphrase: String,        // default: "changeme"
    pub secret_key: String,        // default: "secret"
    pub git_path: String,          // default: "/usr/bin/git"
    pub bucket_addr: String,       // default: "localhost:9000"
    pub bucket_access_key: String, // default: "minioadmin"
    pub bucket_secret_key: String, // default: "minioadmin"
    pub bucket_name: String,       // default: "sealci"

    // Example: http://hugo.fr
    pub scheduler_host: String, // default: "http://localhost"
    // Example: 8080
    pub scheduler_port: String, // default: "8080"

    // Example: http://hugo.fr
    pub agent_host: String, // default: "http://localhost"
    // Example: 8080
    pub agent_port: u32, // default: 8080
}

impl Default for GlobalConfig {
    fn default() -> Self {
        GlobalConfig {
            monitor_port: "4444".to_string(),
            controller_host: "http://0.0.0.0".to_string(),
            controller_port: "4445".to_string(),
            database_url: "postgres://postgres:postgres@0.0.0.0:5432/sealci".to_string(),
            release_agent_host: "http://192.168.1.2".to_string(),
            release_agent_port: "4446".to_string(),
            passphrase: "changeme".to_string(),
            secret_key: "secret".to_string(),
            git_path: "/usr/bin/git".to_string(),
            bucket_addr: "http://192.168.1.1:9000".to_string(),
            bucket_access_key: "minioadmin".to_string(),
            bucket_secret_key: "minioadmin".to_string(),
            bucket_name: "sealci".to_string(),
            scheduler_host: "http://localhost".to_string(),
            scheduler_port: "4447".to_string(),
            agent_host: "http://localhost".to_string(),
            agent_port: 4448,
        }
    }
}

impl Update<AgentMutation> for GlobalConfig {
    fn update(&mut self, mutation: AgentMutation) {
        if let Some(ahost) = mutation.agent_host {
            self.agent_host = ahost;
        }
        if let Some(port) = mutation.agent_port {
            self.agent_port = port;
        }
    }
}

impl Update<ControllerMutation> for GlobalConfig {
    fn update(&mut self, mutation: ControllerMutation) {
        if let Some(host) = mutation.controller_host {
            self.controller_host = host;
        }
        if let Some(port) = mutation.controller_port {
            self.controller_port = port;
        }
        if let Some(database_url) = mutation.database_url {
            self.database_url = database_url;
        }
    }
}

impl Update<MonitorMutation> for GlobalConfig {
    fn update(&mut self, mutation: MonitorMutation) {
        if let Some(port) = mutation.monitor_port {
            self.monitor_port = port.to_string();
        }
    }
}

impl Update<SchedulerMutation> for GlobalConfig {
    fn update(&mut self, mutation: SchedulerMutation) {
        if let Some(host) = mutation.scheduler_host {
            self.scheduler_host = host;
        }
        if let Some(port) = mutation.scheduler_port {
            self.scheduler_port = port;
        }
    }
}

impl Update<ReleaseAgentMutation> for GlobalConfig {
    fn update(&mut self, mutation: ReleaseAgentMutation) {
        if let Some(host) = mutation.release_agent_host {
            self.release_agent_host = host;
        }
        if let Some(port) = mutation.release_agent_port {
            self.release_agent_port = port;
        }
        if let Some(passphrase) = mutation.passphrase {
            self.passphrase = passphrase;
        }
        if let Some(secret_key) = mutation.secret_key {
            self.secret_key = secret_key;
        }
        if let Some(git_path) = mutation.git_path {
            self.git_path = git_path;
        }
        if let Some(bucket_addr) = mutation.bucket_addr {
            self.bucket_addr = bucket_addr;
        }
        if let Some(bucket_access_key) = mutation.bucket_access_key {
            self.bucket_access_key = bucket_access_key;
        }
        if let Some(bucket_secret_key) = mutation.bucket_secret_key {
            self.bucket_secret_key = bucket_secret_key;
        }
        if let Some(bucket_name) = mutation.bucket_name {
            self.bucket_name = bucket_name;
        }
    }
}

impl Into<agent::config::Config> for GlobalConfig {
    fn into(self) -> agent::config::Config {
        agent::config::Config {
            shost: self.scheduler_host + ":" + &self.scheduler_port,
            ahost: self.agent_host,
            port: self.agent_port,
        }
    }
}

impl Into<controller::config::Config> for GlobalConfig {
    fn into(self) -> controller::config::Config {
        controller::config::Config {
            http: format!("0.0.0.0:{}", self.controller_port),
            database_url: self.database_url,
            grpc: self.scheduler_host + ":" + &self.scheduler_port,
            release_agent: self.release_agent_host + ":" + &self.release_agent_port,
        }
    }
}

impl Into<monitor::config::Config> for GlobalConfig {
    fn into(self) -> monitor::config::Config {
        monitor::config::Config {
            controller_host: format!("{}:{}", self.controller_host, self.controller_port),
            port: self.monitor_port.parse().unwrap_or(9001),
        }
    }
}

impl Into<sealci_scheduler::config::Config> for GlobalConfig {
    fn into(self) -> sealci_scheduler::config::Config {
        sealci_scheduler::config::Config {
            addr: format!("0.0.0.0:{}", self.scheduler_port),
        }
    }
}

impl Into<compactor::config::Config> for GlobalConfig {
    fn into(self) -> compactor::config::Config {
        compactor::config::Config {
            image: "milou666/release-agent:hugo".to_string(),
            tap_interface_name: "tap0".to_string(),
            interactive: false,
            env: vec![
                format!("DEBUG={}", "true"),
                format!("LOG_LEVEL={}", "info"),
                format!("CERT_PATH={}", "/tmp"),
                format!("GIT_PATH={}", self.git_path),
                format!("BUCKET_ACCESS_KEY={}", self.bucket_access_key),
                format!("BUCKET_SECRET_KEY={}", self.bucket_secret_key),
                format!("BUCKET_NAME={}", self.bucket_name),
                format!("BUCKET_ADDR={}", self.bucket_addr),
                format!("GRPC=0.0.0.0:{}", &self.release_agent_port),
            ],
            transfer_files: vec![],
            mem_size_mb: 2048,
            num_vcpus: 1,
            dns: None,
        }
    }
}
