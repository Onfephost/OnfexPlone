# System Design — Project Name

> One-line summary of what this system does and who it is for.

## Architecture

```mermaid
flowchart TD
    U([User]) --> APP[Web & Mobile App]
    APP --> API[API Gateway]
    API --> AUTH[Auth Service]
    API --> CORE[Core Service]
    CORE --> DB[(Database)]
    CORE --> CACHE[(Cache)]
    CORE --> QUEUE[[Job Queue]]
    QUEUE --> WORKER[Background Workers]
```

## Request Flow

```mermaid
sequenceDiagram
    participant U as User
    participant A as API
    participant C as Core
    participant D as Database
    U->>A: Request
    A->>C: Validate & route
    C->>D: Read / write
    D-->>C: Result
    C-->>A: Response
    A-->>U: 200 OK
```

## Components

| Component    | Responsibility             |
| ------------ | -------------------------- |
| API Gateway  | Routing, auth, rate limits |
| Core Service | Business logic             |
| Database     | Source of truth            |
| Cache        | Fast reads                 |
| Workers      | Async / scheduled jobs     |

## Open Questions

- [ ] Scaling strategy for peak load?
- [ ] Data retention & backup policy?
- [ ] Monitoring & alerting plan?

> **Tip:** tap any diagram above to edit it in the Mermaid Studio.
