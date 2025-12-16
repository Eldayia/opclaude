# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Opcode is a desktop application that provides a GUI for managing Claude Code sessions, creating custom agents, tracking usage, and managing MCP servers. It's built as a Tauri 2 application with a React frontend and Rust backend.

## Build & Development Commands

```bash
# Install dependencies
bun install

# Development mode with hot reload
bun run tauri dev

# Frontend only (no Tauri)
bun run dev

# Production build
bun run tauri build

# Type checking (frontend + Rust)
bun run check

# Run Rust tests
cd src-tauri && cargo test

# Format Rust code
cd src-tauri && cargo fmt
```

## Architecture

### Tech Stack
- **Frontend**: React 18 + TypeScript + Vite 6
- **Backend**: Rust with Tauri 2
- **UI**: Tailwind CSS v4 + shadcn/ui components
- **State Management**: Zustand
- **Database**: SQLite (rusqlite)
- **Package Manager**: Bun

### Frontend Structure (src/)
- `App.tsx` - Main application entry, handles routing between views
- `components/` - React components
  - `ui/` - Reusable shadcn/ui components
  - `widgets/` - Tool-specific widgets (Bash, LS, Todo)
  - `ClaudeCodeSession.tsx` - Interactive Claude Code session UI with streaming output
  - `CCAgents.tsx` - Agent management interface
  - `TabManager.tsx` / `TabContent.tsx` - Multi-tab session handling
- `lib/api.ts` - TypeScript API client that calls Rust backend via Tauri invoke
- `lib/apiAdapter.ts` - Adapter that switches between Tauri and web mode
- `stores/` - Zustand stores for state management
- `contexts/` - React contexts (TabContext, ThemeContext)

### Backend Structure (src-tauri/)
- `src/main.rs` - Tauri app entry point, registers all command handlers
- `src/lib.rs` - Module declarations
- `src/commands/` - Tauri command handlers:
  - `claude.rs` - Project/session management, Claude Code execution with streaming
  - `agents.rs` - CC Agent CRUD, execution, and process management
  - `mcp.rs` - MCP server configuration management
  - `usage.rs` - Usage analytics and cost tracking
  - `storage.rs` - SQLite database operations
  - `slash_commands.rs` - Custom slash command management
  - `proxy.rs` - API proxy configuration
- `src/checkpoint/` - Session timeline/checkpoint system for version control
- `src/process/` - Process registry for tracking running Claude sessions
- `src/claude_binary.rs` - Claude binary discovery across different installations
- `src/web_server.rs` - Web server mode (alternative to desktop app)

### Data Flow
1. Frontend calls `api.*` methods in `lib/api.ts`
2. `apiAdapter.ts` routes to Tauri invoke (desktop) or HTTP (web mode)
3. Rust command handlers in `src-tauri/src/commands/` process requests
4. Commands interact with `~/.claude/` directory for Claude Code data
5. Agent data stored in SQLite database at app data directory
6. Streaming output uses Tauri events (`agent-output`, `claude-output`)

### Key Patterns
- All Tauri commands use `#[tauri::command]` attribute and async
- Frontend receives streaming output via Tauri event listeners
- Process registry (`process/registry.rs`) tracks running Claude/agent processes
- Claude sessions stored as JSONL files in `~/.claude/projects/{encoded-path}/`
