# Container Runtime Migration Plan

## Context

The phenotype ecosystem currently relies on Docker Desktop for local containerized development. Docker Desktop has several pain points:
- High memory/CPU usage
- Slow startup times
- Requires admin privileges
- Paid subscription for commercial use

## Current State Analysis

### Docker Usage Across Projects

| Project | Docker Usage | Primary Purpose |
|--------|-------------|-----------------|
| `heliosCLI` | `Dockerfile`, `docker-compose.yaml` | Local dev environment |
| `AgilePlus` | `Dockerfile.rust`, `docker-compose.yml` | Python/Rust dev environments |
| `bifrost-extensions` | `Dockerfile`, `docker-compose.yml` | SLM server deployment |
| `trace` | Multiple Dockerfiles, docker-compose files | Chaos engineering, testing |
| `cliproxyapi++` | `Dockerfile`, `docker-compose.yml` | API service containerization |
| `helios-cli` | `.devcontainer/Dockerfile` | VS Code dev containers |

### Archive Count
- **59+ Docker-related files** found across active worktrees
- Multiple duplicate Dockerfiles for the same purpose across worktrees

## Solution Landscape

### Layer 1: Native Container Runtimes (Fastest, Lightest)

| Platform | Solution | Pros | Cons |
|----------|----------|------|------|
| **macOS** | **OrbStack** | 10x faster, <0.1% CPU idle, drop-in Docker replacement | macOS only |
| **macOS** | **Colima** | Open source, similar to OrbStack | Less polished UI |
| **Linux** | **Docker/Podman native** | Direct kernel access, fastest | Requires native Linux |
| **Windows** | **Docker Desktop WSL2** | Best Linux container support | High resource usage |
| **Windows** | **Podman Desktop + WSL2** | Open source alternative | Less mature than Docker |

### Layer 2: Lightweight VMs (For full Linux environments)

| Platform | Solution | Pros | Cons |
|----------|----------|------|------|
| **macOS** | **OrbStack Linux Machines** | Seamless containers + VMs, single app | Proprietary |
| **macOS** | **Multipass** | Ubuntu-first, cloud-init support | Limited distros |
| **macOS** | **UTM** | Full VM support, macOS guest | Manual Docker setup |
| **Windows** | **WSL2** | Native Linux kernel, fast | Windows-specific |
| **Linux** | **LXD/LXC** | System containers, near-native | Complex setup |

### Layer 3: Traditional VMs (For isolation/bare metal)

| Platform | Solution | Pros | Cons |
|----------|----------|------|------|
| **All** | **VirtualBox** | Cross-platform, familiar | Slow, heavy |
| **All** | **VMware Fusion/Workstation** | Better perf than VB | Paid, proprietary |
| **All** | **QEMU/libvirt** | Open source, flexible | Complex config |

### Layer 4: Minimal/No Container (Past approaches)

| Approach | Use Case | Tools |
|----------|----------|-------|
| **Native dev** | Simple projects | Direct install (brew, apt) |
| **Language managers** | Multi-version runtime | mise, asdf, pyenv |
| **Remote dev** | CI/CD heavy projects | GitHub Codespaces, SSH |

## Recommended Migration Strategy

### macOS (Primary Development Platform)

```
RECOMMENDED: OrbStack
├── Containers (Docker compatible)
│   └── `docker` CLI works directly
├── Linux Machines (optional)
│   └── Full Ubuntu/Debian VMs
└── Kubernetes (optional)
    └── k3s integration
```

**Migration Steps:**
1. Install OrbStack: `brew install orbstack`
2. Remove Docker Desktop
3. Verify `docker` CLI compatibility
4. Update any Docker Desktop-specific configs (e.g., socket paths)

### Windows (Secondary Development)

```
RECOMMENDED: Docker Desktop + WSL2 (until Podman matures)
ALTERNATIVE: Podman Desktop + WSL2 backend
```

**Migration Steps:**
1. Keep Docker Desktop with WSL2 backend
2. Monitor Podman Desktop development
3. Evaluate migration when Podman Desktop reaches feature parity

### Linux (CI/CD, Servers)

```
RECOMMENDED: Native Docker or Podman
├── Podman (rootless, daemonless)
└── Docker (if OCI compatibility required)
```

**Migration Steps:**
1. Use native packages (apt, dnf, pacman)
2. Enable Docker/Podman socket for local development
3. Consider rootless mode for security

## Docker Compose Migration Matrix

### OrbStack/Colima Compatibility

| Feature | Status | Notes |
|---------|--------|-------|
| `docker compose up` | ✅ Works | Drop-in replacement |
| `docker-compose.yml` | ✅ Works | Full compatibility |
| BuildKit | ✅ Works | Native support |
| Multi-platform builds | ✅ Works | Rosetta x86 emulation |
| GPU access | ⚠️ Limited | Requires additional config |

### Migration Commands

```bash
# Stop Docker Desktop
# Install OrbStack
brew install orbstack

# Verify installation
docker run hello-world

# Migrate compose files (no changes needed)
docker compose -f docker-compose.yml up -d

# For GPU workloads
orbstack config set gpu enabled
```

## Implementation Phases

### Phase 1: Research & Planning (Completed)
- [x] Survey Docker usage across all projects
- [x] Research container alternatives
- [x] Evaluate OrbStack/Colima/Podman

### Phase 2: Pilot Migration (Week 1-2)
- [ ] Install OrbStack on primary development machine
- [ ] Test with 1-2 low-risk projects (e.g., `heliosCLI`)
- [ ] Document any compatibility issues
- [ ] Create runbook for team

### Phase 3: Staged Rollout (Week 3-4)
- [ ] Migrate remaining projects
- [ ] Update CI/CD if needed (GitHub Actions use Docker directly)
- [ ] Archive duplicate Dockerfiles
- [ ] Update documentation

### Phase 4: Cleanup (Week 5+)
- [ ] Remove Docker Desktop
- [ ] Consolidate Dockerfiles
- [ ] Update `CONTRIBUTING.md` with new setup instructions
- [ ] Create onboarding script

## Project-Specific Recommendations

### heliosCLI
- **Current:** Multiple Dockerfiles for different Rust versions
- **Recommendation:** Use OrbStack + native Rust (via rustup)
- **Dockerfile use:** Keep for CI/CD only

### AgilePlus
- **Current:** Python + Rust Docker setup
- **Recommendation:** OrbStack for containerized services, native for dev
- **Dockerfile use:** Production builds only

### bifrost-extensions
- **Current:** SLM server in Docker
- **Recommendation:** OrbStack Linux Machine for SLM
- **Alternative:** Native deployment on OrbStack Linux VM

### trace
- **Current:** Complex docker-compose for chaos engineering
- **Recommendation:** Keep Docker/Podman, evaluate OrbStack
- **Note:** Chaos engineering may have specific Docker requirements

## Risks & Mitigations

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| OrbStack compatibility issues | Low | Medium | Test with pilot projects first |
| CI/CD pipeline breakage | Medium | High | Keep Dockerfiles for CI, test thoroughly |
| Team learning curve | Low | Low | OrbStack is drop-in replacement |
| Proprietary lock-in | Low | Medium | Document migration path to pure Docker |

## Cost Analysis

| Solution | Cost | 3-Year TCO |
|----------|------|------------|
| Docker Desktop | $0-120/yr | $360 |
| OrbStack | $0-120/yr | $360 |
| Colima | $0 | $0 |
| Podman Desktop | $0 | $0 |

*Note: OrbStack has a free tier for personal/hobby use*

## References

- [OrbStack Documentation](https://orbstack.dev/docs)
- [OrbStack vs Docker Desktop](https://orbstack.dev/docs/compare/docker-desktop)
- [OrbStack vs Colima](https://orbstack.dev/docs/compare/colima)
- [Colima GitHub](https://github.com/abiosoft/colima)
- [Podman Desktop](https://podman-desktop.io/)
- [Multipass](https://multipass.run/)

## Appendix: Quick Reference Commands

```bash
# Install OrbStack
brew install orbstack

# Check Docker is working
docker info

# List running containers
docker ps

# Start a container
docker run -it ubuntu bash

# Docker Compose (v2 syntax)
docker compose up -d

# OrbStack specific
orbstack list machines
orbstack start ubuntu
```

---

*Created: 2026-03-25*
*Author: Infrastructure Team*
*Status: Draft - Awaiting Review*
