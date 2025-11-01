# Contributing to NetVisor

Thank you for your interest in contributing to NetVisor! We welcome contributions of all kinds, from bug reports and documentation improvements to new features and service definitions.

## Table of Contents

- [Getting Started](#getting-started)
- [Ways to Contribute](#ways-to-contribute)
- [Development Environment Setup](#development-environment-setup)
- [Development Workflow](#development-workflow)
- [Adding Service Definitions](#adding-service-definitions)
- [Testing](#testing)
- [Submitting Your Contribution](#submitting-your-contribution)

## Getting Started

### Quick Start for Service Definitions

**The easiest way to contribute is by adding service definitions!** Service definitions help NetVisor identify and categorize network services during discovery. This is a great first contribution that doesn't require deep knowledge of the codebase. Given the wide variety of services that folks run across their networks, this is inherently best handled as a community-driven effort.

If you're interested in adding a service definition, jump to the [Adding Service Definitions](#adding-service-definitions) section.

## Ways to Contribute

### 1. Service Definitions (Recommended for First-Time Contributors)

Service definitions are small, focused additions that help NetVisor discover and identify specific services on your network. Examples include:

- Home automation platforms (Home Assistant, OpenHAB)
- Media servers (Plex, Jellyfin, Emby)
- Infrastructure services (Pi-hole, AdGuard, Traefik)
- Development tools (Portainer, Grafana, Jenkins)

### 2. Bug Reports

Found a bug? [Please open an issue!](https://github.com/mayanayza/netvisor/issues/new?template=bug_report.md)

### 3. Documentation

Help improve our documentation:

- Fix typos or clarify existing docs
- Add examples or tutorials for specific setups
- Improve installation instructions
- Document troubleshooting steps

### 4. Code Contributions

For larger features or bug fixes:

- Discuss your idea in an issue first
- Follow the development workflow below
- Write tests for new functionality
- Update documentation as needed

## Development Environment Setup

### Prerequisites

**For Daemon Development:**

- Linux: Docker with host networking support, OR binary installation
- Mac/Windows: Binary installation only (Docker Desktop does not support host networking)

**For Server Development:**

- Rust 1.90 or later
- Node.js 20 or later
- PostgreSQL 17
- Docker and Docker Compose (optional, for containerized development)

### Initial Setup

1. **Clone the repository**

   ```bash
   git clone https://github.com/mayanayza/netvisor.git
   cd netvisor
   ```

2. **Install development dependencies**

    On Ubuntu/Debian:
   1. Install NVM and Node.js 20

        ```bash
        curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.0/install.sh | bash
        nvm install 20
        nvm use 20
        ```

   2. Install postgresql-17

        ```bash
        sudo apt install curl ca-certificates gnupg2 wget vim -y
        sudo install -d /usr/share/postgresql-common/pgdg
        sudo curl -o /usr/share/postgresql-common/pgdg/apt.postgresql.org.asc --fail https://www.postgresql.org/media/keys/ACCC4CF8.asc
        sudo sh -c 'echo "deb [arch=amd64 signed-by=/usr/share/postgresql-common/pgdg/apt.postgresql.org.asc] https://apt.postgresql.org/pub/repos/apt $(lsb_release -cs)-pgdg main" > /etc/apt/sources.list.d/pgdg.list'
        sudo apt update
        sudo apt -y install postgresql-17
        ```

   3. Install project dependencies

        ```bash
        npm i --save-dev prettier-plugin-svelte prettier prettier-plugin-tailwindcss
        
        make install-dev-linux
        ```

    On MacOS:
   1. Install Homebrew if not already installed

        ```bash
        /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
        ```

   2. Install Rust, Node.js 20, and PostgreSQL 17

        ```bash
        brew install rust node@20 postgresql@17
        ```

   3. Install project dependencies

        ```bash
        make install-dev-mac
        ```

        This installs:
        - Rust toolchain with rustfmt and clippy
        - Node.js dependencies

3. **Set up the database**

   ```bash
   make setup-db
   ```

   This starts a PostgreSQL container on port 5432.

### Development Environments

You have two options for development:

#### Option 1: Local Development (Recommended)

Run components individually with hot reload:

```bash
# Terminal 1 - Start the server
make dev-server

# Terminal 2 - Start the UI
make dev-ui

# Terminal 3 - Start the daemon (if needed)
make dev-daemon
```

**Advantages:**

- Faster iteration with hot reload
- Easier debugging
- More control over individual components

#### Option 2: Containerized Development

Run everything in Docker containers:

```bash
# Start all services
make dev-container

# Rebuild containers
make dev-container-rebuild

# Clean rebuild (no cache)
make dev-container-rebuild-clean

# Stop all services
make dev-down
```

**Use this when:**

- Testing the full stack together
- You want a production-like environment
- You're having dependency issues locally

### Accessing the Application

Once running:

- **UI**: <http://localhost:5173> (with hot reload)
- **Server API**: <http://localhost:60072>
- **Daemon API**: <http://localhost:60073>

## Development Workflow

### Before You Start

1. Create a new branch for your work:

   ```bash
   git checkout -b feature/your-feature-name
   # or
   git checkout -b fix/your-bug-fix
   ```

2. If working on the server/daemon, ensure fresh start:

   ```bash
   make clean-daemon  # Clear daemon config
   make clean-db      # Stop and remove database
   make setup-db      # Create fresh database
   ```

### During Development

1. **Write your code**
   - Follow existing code patterns
   - Add comments for complex logic
   - Keep changes focused and atomic

2. **Test your changes**

   ```bash
   make test
   ```

   Note - this will tear down all containers, including the PostgreSql container; you'll need to recreate that after running.

   You can dump the DB if you want to hold on to the data and reload the container from the dump.

   ```bash
   make dump-db
   ```

3. **Format your code**

   ```bash
   make format
   ```

4. **Lint your code**

   ```bash
   make lint
   ```

### Before Submitting

**Always run these commands before creating a PR:**

```bash
make format  # Format all code
make lint    # Check for issues
make test    # Run all tests
```

All three commands must pass without errors before submitting your PR.

## Adding Service Definitions

Service definitions are the best place to start contributing! They help NetVisor identify and categorize services during network discovery.

### Project Structure

Service definitions are located in:

```
backend/src/server/services/definitions/
├── mod.rs                 # Module registry
├── home_assistant.rs      # Example service definition
├── plex.rs                # Example service definition
└── your_service.rs        # Your new service definition
```

### Step 1: Create Your Service File

Create a new file in `backend/src/server/services/definitions/` named after your service (e.g., `grafana.rs`):

```rust
use crate::server::hosts::types::ports::PortBase;
use crate::server::services::definitions::{create_service, ServiceDefinitionFactory};
use crate::server::services::types::categories::ServiceCategory;
use crate::server::services::types::definitions::ServiceDefinition;
use crate::server::services::types::patterns::Pattern;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Grafana;

impl ServiceDefinition for Grafana {
    fn name(&self) -> &'static str { 
        "Grafana" 
    }
    
    fn description(&self) -> &'static str { 
        "Metrics dashboard and visualization platform" 
    }
    
    fn category(&self) -> ServiceCategory { 
        ServiceCategory::Monitoring 
    }
    
    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Endpoint(
            PortBase::Http,
            "/api/health",
            "grafana"
        )
    }
    
    fn dashboard_icons_path(&self) -> &'static str { 
        "grafana"
    }
}

// This macro registers your service for automatic discovery
inventory::submit!(ServiceDefinitionFactory::new(create_service::<Grafana>));
```

### Step 2: Register the Module

Add your module to `backend/src/server/services/definitions/mod.rs`:

```rust
pub mod grafana;  // Add this line
```

That's it! Your service will now be automatically discovered during network scans.

### Understanding Pattern Types

Patterns define how NetVisor identifies your service. 

Here are the available pattern types:

#### Endpoint Patterns

This is the preferred match type, as the existence of the name of the service in a response is a strong signal that it is in fact the service in question.

**Pattern::Endpoint**
Check if an endpoint returns expected content:

```rust
fn discovery_pattern(&self) -> Pattern<'_> {
    Pattern::Endpoint(
        PortBase::Http,           // Port to check
        "/api/service",            // Path
        "service_name"                 // Expected text in response
    )
}
```

#### Simple Port Patterns

This pattern is acceptable if there are no usable endpoints (ie they require authentication, SSL, or otherwise don't provide service-identifying information), but try to create a pattern with multiple unique ports or combine ports with other information to make the match more precise.

**Pattern::Port**
Match a specific port:

```rust
fn discovery_pattern(&self) -> Pattern<'_> {
    Pattern::Port(PortBase::Http)  // Port 80
}
```

Common PortBase values:

- `PortBase::Http` (80)
- `PortBase::Https` (443)
- `PortBase::HttpAlt` (8080)
- `PortBase::Ssh` (22)
- `PortBase::DnsUdp` (53)
- For custom ports: `PortBase::new_tcp(8000)` or `PortBase::new_udp(1900)`

**Note** UDP pattern matching is barely supported outside of DNS and a few others. Please don't rely heavily on UDP ports.

#### Logical Patterns

**Pattern::AnyOf**
Match if ANY pattern succeeds:

```rust
fn discovery_pattern(&self) -> Pattern<'_> {
    Pattern::AnyOf(vec![
        Pattern::Port(PortBase::new_tcp(32400)),
        Pattern::Endpoint(PortBase::Http, "/web", "Plex")
    ])
}
```

**Pattern::AllOf**
Match ONLY if ALL patterns succeed:

```rust
fn discovery_pattern(&self) -> Pattern<'_> {
    Pattern::AllOf(vec![
        Pattern::Port(PortBase::Http),
        Pattern::Port(PortBase::new_tcp(8443))
    ])
}
```

**Pattern::Not**
Inverse of a pattern:

```rust
fn discovery_pattern(&self) -> Pattern<'_> {
    Pattern::Not(&Pattern::IsGateway)
}
```

#### Special Patterns

**Pattern::IsGateway**
Matches if the host is in the routing table as a gateway:

```rust
fn discovery_pattern(&self) -> Pattern<'_> {
    Pattern::IsGateway
}
```

**Pattern::MacVendor**
Match based on MAC address vendor:

```rust
use crate::server::services::types::patterns::Vendor;

fn discovery_pattern(&self) -> Pattern<'_> {
    Pattern::MacVendor(Vendor::EERO)
}
```

To add new Vendor:: values:

1. Go to `backend/src/server/services/types/patterns.rs` and ctrl+f "pub struct Vendor;"
2. Use `https://gist.github.com/aallan/b4bb86db86079509e6159810ae9bd3e4` to identify the string used by a vendor for their MAC address patterns.
3. Add your new Vendor value: 

```rust 
pub const NEWVENDOR: &'static str = "Acme, Inc"
```;

**Pattern::SubnetIsType**
Match based on subnet type:
```rust
use crate::server::subnets::types::base::SubnetType;

fn discovery_pattern(&self) -> Pattern<'_> {
    Pattern::SubnetIsType(SubnetType::Guest)
}
```

For a list of subnet types and information on how they are derived, check out `backend/src/server/subnets/types/base.rs`. 
`pub enum SubnetType` has the list, and the method `from_interface_name` has specifics on how they are matched.

**Pattern::None**
For services that aren't auto-discovered (manual only):

```rust
fn discovery_pattern(&self) -> Pattern<'_> {
    Pattern::None
}
```

### Service Categories

Choose the most appropriate category. If the service you want to add doesn't fit the category, you can add one at `backend/src/server/services/types/categories.rs`.

#### Infrastructure

- `ServiceCategory::NetworkCore` - Switches, core infrastructure
- `ServiceCategory::NetworkAccess` - Routers, access points
- `ServiceCategory::NetworkSecurity` - Firewalls, security appliances
- `ServiceCategory::DNS` - DNS servers
- `ServiceCategory::VPN` - VPN servers
- `ServiceCategory::ReverseProxy` - Nginx, Traefik, HAProxy, etc

#### Server Services

- `ServiceCategory::Storage` - NAS, file servers
- `ServiceCategory::Media` - Plex, Jellyfin, Emby
- `ServiceCategory::HomeAutomation` - Home Assistant, OpenHAB
- `ServiceCategory::Virtualization` - Proxmox, VMware, Docker
- `ServiceCategory::Backup` - Backup services

#### Applications

- `ServiceCategory::Web` - Web servers and applications
- `ServiceCategory::Database` - Database servers
- `ServiceCategory::Development` - Development tools
- `ServiceCategory::Dashboard` - Dashboards, admin panels
- `ServiceCategory::Monitoring` - Monitoring and metrics

#### Devices

- `ServiceCategory::Workstation` - Desktop computers
- `ServiceCategory::Mobile` - Mobile devices
- `ServiceCategory::IoT` - IoT devices
- `ServiceCategory::Printer` - Printers

#### Other

- `ServiceCategory::AdBlock` - Pi-hole, AdGuard
- `ServiceCategory::Custom` - Custom services
- `ServiceCategory::Unknown` - When unclear

### Optional Properties

#### Generic Services
Mark services not tied to a specific brand.

```rust
fn is_generic(&self) -> bool { 
    true 
}
```

#### Service Icons

NetVisor supports icons from three sources. All icons **must support SVG format**.

**Dashboard Icons** (Recommended - has the most service icons):

```rust
fn dashboard_icons_path(&self) -> &'static str { 
    "home-assistant"  // From dashboardicons.com/icons/home-assistant
}
```

**Simple Icons**:

```rust
fn simple_icons_path(&self) -> &'static str { 
    "plex"  // From simpleicons.org/icons/plex.svg
}
```

**Vector Logo Zone**:

```rust
fn vector_logo_zone_icons_path(&self) -> &'static str { 
    "akamai/akamai-icon"  // From vectorlogo.zone/logos/akamai/akamai-icon.svg
}
```

**White Background** (for dark logos):

```rust
fn logo_needs_white_background(&self) -> bool {
    true
}
```

Browse available icons:

- Dashboard Icons: <https://dashboardicons.com/>
- Simple Icons: <https://simpleicons.org/>
- Vector Logo Zone: <https://www.vectorlogo.zone/>

### Complete Examples

#### Simple Port-Based Service

```rust
use crate::server::hosts::types::ports::PortBase;
use crate::server::services::definitions::{create_service, ServiceDefinitionFactory};
use crate::server::services::types::categories::ServiceCategory;
use crate::server::services::types::definitions::ServiceDefinition;
use crate::server::services::types::patterns::Pattern;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Redis;

impl ServiceDefinition for Redis {
    fn name(&self) -> &'static str { "Redis" }
    fn description(&self) -> &'static str { "In-memory data structure store" }
    fn category(&self) -> ServiceCategory { ServiceCategory::Database }
    
    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Port(PortBase::new_tcp(6379))
    }
    
    fn simple_icons_path(&self) -> &'static str { "redis" }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Redis>));
```

#### Web Service with Health Check

```rust
use crate::server::hosts::types::ports::PortBase;
use crate::server::services::definitions::{create_service, ServiceDefinitionFactory};
use crate::server::services::types::categories::ServiceCategory;
use crate::server::services::types::definitions::ServiceDefinition;
use crate::server::services::types::patterns::Pattern;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Portainer;

impl ServiceDefinition for Portainer {
    fn name(&self) -> &'static str { "Portainer" }
    fn description(&self) -> &'static str { "Docker container management interface" }
    fn category(&self) -> ServiceCategory { ServiceCategory::Virtualization }
    
    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::Endpoint(
            PortBase::HttpAlt,
            "/api/status",
            "Portainer"
        )
    }
    
    fn dashboard_icons_path(&self) -> &'static str { "portainer" }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<Portainer>));
```

#### Complex Multi-Pattern Service

```rust
use crate::server::hosts::types::ports::PortBase;
use crate::server::services::definitions::{create_service, ServiceDefinitionFactory};
use crate::server::services::types::categories::ServiceCategory;
use crate::server::services::types::definitions::ServiceDefinition;
use crate::server::services::types::patterns::Pattern;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct HomeAssistant;

impl ServiceDefinition for HomeAssistant {
    fn name(&self) -> &'static str { "Home Assistant" }
    fn description(&self) -> &'static str { "Open-source home automation platform" }
    fn category(&self) -> ServiceCategory { ServiceCategory::HomeAutomation }
    
    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::AnyOf(vec![
            // Check API endpoint
            Pattern::Endpoint(
                PortBase::HttpAlt,
                "/api/",
                "Home Assistant"
            ),
            // Or check default port with web response
            Pattern::AllOf(vec![
                Pattern::Port(PortBase::new_tcp(8123)),
                Pattern::Endpoint(PortBase::Http, "/", "homeassistant")
            ])
        ])
    }
    
    fn dashboard_icons_path(&self) -> &'static str { "home-assistant" }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<HomeAssistant>));
```

#### Gateway Service with MAC Vendor

```rust
use crate::server::services::definitions::{create_service, ServiceDefinitionFactory};
use crate::server::services::types::categories::ServiceCategory;
use crate::server::services::types::definitions::ServiceDefinition;
use crate::server::services::types::patterns::{Pattern, Vendor};

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct EeroGateway;

impl ServiceDefinition for EeroGateway {
    fn name(&self) -> &'static str { "Eero Gateway" }
    fn description(&self) -> &'static str { "Eero mesh WiFi router" }
    fn category(&self) -> ServiceCategory { ServiceCategory::NetworkAccess }
    
    fn discovery_pattern(&self) -> Pattern<'_> {
        Pattern::AllOf(vec![
            Pattern::MacVendor(Vendor::EERO),
            Pattern::IsGateway
        ])
    }
    
    fn vector_logo_zone_icons_path(&self) -> &'static str { "eero/eero-icon" }
    fn logo_needs_white_background(&self) -> bool { true }
}

inventory::submit!(ServiceDefinitionFactory::new(create_service::<EeroGateway>));
```

## Testing

### Running Tests

Before submitting any PR, you must run all tests:

```bash
make test
```

This will:

- Stop any running dev containers
- Clean daemon config
- Run all backend and integration tests

### Testing Your Service Definition

#### 1. Verify Compilation

```bash
make dev-server
```

Check the server logs for any compilation errors.

#### 2. Test Discovery

If you have the actual service running on your network:

1. Start NetVisor with your changes
2. Navigate to the discovery page in the UI
3. Run a network scan
4. Verify your service is detected and correctly categorized
5. Check that the icon displays correctly

#### 3. Manual Testing

Even if you don't have the service running, you should verify:

- The service compiles without errors
- The pattern logic makes sense
- The category is appropriate
- The icon loads correctly

### Writing Tests (Optional but Appreciated)

If you're adding complex logic, consider adding unit tests:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_properties() {
        let service = YourService;
        assert_eq!(service.name(), "Your Service");
        assert_eq!(service.category(), ServiceCategory::Web);
    }
}
```

## Submitting Your Contribution

### Before You Submit

**Pre-submission checklist:**

- [ ] Created a descriptive branch name
- [ ] Code follows existing patterns and conventions
- [ ] Ran `make format` to format all code
- [ ] Ran `make lint` with no errors
- [ ] Ran `make test` with all tests passing
- [ ] Tested your changes (if possible)
- [ ] Updated documentation (if needed)
- [ ] Committed with clear, descriptive messages

### Pull Request Guidelines

1. **One change per PR**: Keep PRs focused
   - One service definition per PR
   - One bug fix per PR
   - Related changes can be grouped

2. **Clear title**: Use descriptive titles
   - `Add service definition for Grafana`
   - `Fix port scanning timeout issue`
   - `Update installation documentation`

3. **Good description**: Include context and details
   - What problem does this solve?
   - How did you test it?
   - Any breaking changes?
   - Screenshots (for UI changes)

### PR Template for Service Definitions

```markdown
## Add service definition for [Service Name]

**Description**: [Brief description of what this service does]

**Official Website**: [URL]

**Default Ports**: [List the ports this service typically uses]

**Discovery Method**: [Explain the pattern used and why]
- Pattern type: [Port/Endpoint/Other]
- Reasoning: [Why this pattern is appropriate]

**Icon Source**: [Dashboard Icons/Simple Icons/Vector Logo Zone]

**Testing**: 
- [ ] Compiles successfully
- [ ] Tested against real instance (describe setup below)
- [ ] Unable to test (explain why below)

**Testing Details**: 
[Describe how you tested this, or why you couldn't test it]

**Additional Notes**: 
[Any special considerations, edge cases, or future improvements]
```

### After Submitting

- Make requested changes in new commits (don't force-push)
- Be open to feedback and suggestions

## Getting Help

- **Questions?** Open a discussion on GitHub
- **Stuck?** Comment on your PR or issue

## Code of Conduct

- Be respectful and professional
- Provide constructive feedback
- Help others learn and grow
- Follow the project's coding standards

## License

By contributing to NetVisor, you agree that your contributions will be licensed under the GNU Affero General Public License v3.0 (AGPL-3.0).

---

Thank you for contributing to NetVisor! Every contribution, no matter how small, helps make network discovery and documentation better for everyone.
