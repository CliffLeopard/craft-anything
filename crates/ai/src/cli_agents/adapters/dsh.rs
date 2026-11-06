//! DeepSeek Harness (`dsh --profile acp`).
//!
//! DSH's shipped `acp` profile is a standard ACP v1 stdio server, so the builder and question
//! routes use it instead of the plain prompt-in/text-out runner. The program is normally the
//! CLI inside the installed app bundle; name it with the backend's `command` (or put `dsh` on
//! PATH), and `--profile acp` is appended by [`super::super::acp::ACP_AGENTS`].

use super::*;

pub struct DeepSeekHarness;

impl AgentAdapter for DeepSeekHarness {
    fn ids(&self) -> &'static [&'static str] {
        &["dsh"]
    }

    fn args(&self, _cx: &ArgsCx) -> Vec<String> {
        // The ACP path refuses non-ACP execution before it asks for arguments.
        Vec::new()
    }

    fn requires_acp(&self) -> bool {
        true
    }

    fn plain_fallback(&self) -> bool {
        false
    }

    fn answers_questions(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn dsh_is_registered_and_acp_only() {
        let adapter = super::super::AgentAdapters::built_in().for_agent("dsh");
        assert!(adapter.requires_acp(), "DSH runs over ACP");
        assert!(adapter.answers_questions(), "build questions use its ACP session");
        assert!(adapter.ids().contains(&"dsh"));
    }
}
