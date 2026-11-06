# DeepSeek Harness (DSH)

Craft Anything can use an installed DeepSeek Harness as its coding agent. DSH ships an `acp` profile that serves the Agent Client Protocol over stdio, so Craft Anything starts it like the other ACP agents: one agent process per chat or build, with Craft Anything's MCP server mounted for the run, and approval requests turned into consent questions or the build's approval policy.

## Setup

1. Install DeepSeek Harness; its desktop app includes the `dsh` command-line launcher.
2. Add the backend and point it at the installed launcher. DSH usually lives inside the app bundle rather than on `PATH`:

   ```sh
   craft-anything-cli ai add --kind agent_cli --model dsh "DeepSeek Harness"
   craft-anything-cli ai program "DeepSeek Harness" "<the app's runtime/cli/bin/dsh>"
   ```

3. Choose it in Settings → AI, or make it the builder:

   ```sh
   craft-anything-cli ai builder "DeepSeek Harness"
   ```

ACP is required for DSH: Craft Anything refuses a native or plain run rather than starting DSH without its tools, so no `acp = true` is needed. The program setting must name the launcher because `dsh` is normally not on `PATH`.

## What it gets

- Our MCP server, mounted in ACP's `session/new`, so the builder can call the same study, build and check tools as the other agents.
- One persistent agent process per chat or build; ACP permission requests go through the same approval policy as the other builders (inbox, Auto mode).
- Build questions use the same ACP session; the adapter advertises that it can answer them.
- Sign-in and credentials stay in DSH. Craft Anything never reads or stores them.

## Limits

- DSH's `acp` profile is automation-only: DSH-specific presentation surfaces (its own approval cards, plans, terminal views) are not part of the ACP connection.
- It does not implement ACP `session/load`; a resumed build starts a new DSH session rather than restoring the old one.
- Craft Anything does not query a DSH model catalogue, so its model picker stays out of the way; choose the model and reasoning effort in DSH's own configuration.
- Detection looks for `dsh` on `PATH`; when the launcher lives in the app bundle, `ai program` is required.

## Verified

- Unit tests cover the adapter registration, its ACP-only behaviour and the `--profile acp` startup arguments.
- Live headless build on macOS arm64 with DSH 0.2.2-rc.1: a static website reached Ready with one check pass, and the builder called Craft Anything's MCP tools (`build file write`, `rebuild check`, brief/finding/handoff). Not run on Windows or Linux, and not with the desktop app.
