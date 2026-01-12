# LCN Development Log

This document captures the requirements and prompts used during the AI-assisted development of LCN (Local Computers in Network).

## Initial Requirements

The project name is Local Computers in Network. The idea is to list all the IPs and hostnames belonging to my homelab computers. These homelab computers are just my old laptops and Raspberry Pi each having Ubuntu Server installed in them.

The idea is LCN service will be installed in these homelab machines on OS installation. As part of the LCN service running, it will be running on a port say 7979. It will offer 2 APIs - hostinfo and scanhosts.

**HostInfo API** when called will give following output in the JSON format:
```json
{
   "hostname": "myname",
   "hostipv4": "100.110.110.110"
}
```
where hostname is same as `uname -n` output. The `hostipv4` is the IPv4 of the host.

**ScanHosts API** when called will give following output in the JSON format:
```json
[
    {
       "hostname": "myname",
       "hostipv4": "100.110.110.110"
    },
    {
       "hostname": "myname2",
       "hostipv4": "100.111.110.110"
    }
]
```

This API will scan all the IPs in the local area network that have port `7979` open and call these hosts with `HostInfo` API to get the remote hosts info. It will return the list of all the hosts that are running LCN service.

As part of this, I also want to develop a simple CLI client that can call `ScanHosts` API of the current host and display the results in a human readable and pleasant way like a dashboard.

I also need to develop a script to install the LCN service in the Ubuntu Server hosts that runs on startup and continuously executes.

**Technical Requirements:**
- Develop in Rust language
- APIs to be async APIs

## Development Prompts

### 1. Initial Development
**Prompt:** "Can you go about building it? Think step by step and explain step by step of the design choices being made?"

**Clarifying Questions Asked:**
- Subnet detection: Auto-detect vs manual configuration → Chose auto-detect with later config file option
- CLI display: Simple table vs TUI dashboard → Chose simple table first, TUI later
- Timeout handling: Fast (500ms) vs Balanced (2s) vs Thorough (5s) → Chose balanced 2s with later configuration

**Design Decisions Made:**
- Project structure: Single Cargo crate with multiple binary targets
- Async runtime: Tokio
- HTTP framework: axum
- Network scanning: Direct TCP port scan with concurrent connections (64 max)
- CLI output: clap + comfy-table
- Installation: systemd service

### 2. Cross-Compilation
**Prompt:** "I also have a Raspberry Pi 3 and an old Dell Latitude. Can the binaries be generated with those architecture as well?"

**Targets identified:**
- Raspberry Pi 3: aarch64-unknown-linux-gnu
- Dell Latitude: x86_64-unknown-linux-gnu (same as dev machine)

**Follow-up Prompt:** "Can we use cross for this purpose?"

Used `cross` tool for simplified cross-compilation via Docker containers.

### 3. GitHub Setup
**Prompt:** "I want to check this folder to GitHub. As part of this I want to setup GitHub Actions so that these binaries are automatically generated and available for download. The targets are Intel x86_64 and another one is Raspberry Pi 3."

**Created:**
- GitHub Actions workflow (`.github/workflows/build.yml`)
- Automated builds for both architectures
- Release automation on version tags

## Tools & Technologies

| Component | Choice |
|-----------|--------|
| Language | Rust |
| Async Runtime | Tokio |
| HTTP Framework | axum |
| HTTP Client | reqwest |
| CLI Parser | clap |
| Table Display | comfy-table |
| Cross-compilation | cross |
| CI/CD | GitHub Actions |
| Service Manager | systemd |

## AI Assistant

Developed with assistance from Claude (Opus 4.5) via Claude Code CLI.
