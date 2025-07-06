use gpui::{App, ClipboardItem, PromptLevel, actions};
use neopilot_actions::feedback::FileBugReport;
use system_specs::SystemSpecs;
use util::ResultExt;
use workspace::Workspace;

pub mod feedback_modal;

pub mod system_specs;

actions!(
    neopilot,
    [
        CopySystemSpecsIntoClipboard,
        EmailNeopilot,
        OpenNeopilotRepo,
        RequestFeature,
    ]
);

const NEOPILOT_REPO_URL: &str = "https://github.com/khulnasoft-lab/neopilot";

const REQUEST_FEATURE_URL: &str =
    "https://github.com/khulnasoft-lab/neopilot/discussions/new/choose";

fn file_bug_report_url(specs: &SystemSpecs) -> String {
    format!(
        concat!(
            "https://github.com/khulnasoft-lab/neopilot/issues/new",
            "?",
            "template=10_bug_report.yml",
            "&",
            "environment={}"
        ),
        urlencoding::encode(&specs.to_string())
    )
}

fn email_neopilot_url(specs: &SystemSpecs) -> String {
    format!(
        concat!("mailto:hi@neopilot.dev", "?", "body={}"),
        email_body(specs)
    )
}

fn email_body(specs: &SystemSpecs) -> String {
    let body = format!("\n\nSystem Information:\n\n{}", specs);
    urlencoding::encode(&body).to_string()
}

pub fn init(cx: &mut App) {
    cx.observe_new(|workspace: &mut Workspace, window, cx| {
        let Some(window) = window else {
            return;
        };
        feedback_modal::FeedbackModal::register(workspace, window, cx);
        workspace
            .register_action(|_, _: &CopySystemSpecsIntoClipboard, window, cx| {
                let specs = SystemSpecs::new(window, cx);

                cx.spawn_in(window, async move |_, cx| {
                    let specs = specs.await.to_string();

                    cx.update(|_, cx| {
                        cx.write_to_clipboard(ClipboardItem::new_string(specs.clone()))
                    })
                    .log_err();

                    cx.prompt(
                        PromptLevel::Info,
                        "Copied into clipboard",
                        Some(&specs),
                        &["OK"],
                    )
                    .await
                })
                .detach();
            })
            .register_action(|_, _: &RequestFeature, _, cx| {
                cx.open_url(REQUEST_FEATURE_URL);
            })
            .register_action(move |_, _: &FileBugReport, window, cx| {
                let specs = SystemSpecs::new(window, cx);
                cx.spawn_in(window, async move |_, cx| {
                    let specs = specs.await;
                    cx.update(|_, cx| {
                        cx.open_url(&file_bug_report_url(&specs));
                    })
                    .log_err();
                })
                .detach();
            })
            .register_action(move |_, _: &EmailNeopilot, window, cx| {
                let specs = SystemSpecs::new(window, cx);
                cx.spawn_in(window, async move |_, cx| {
                    let specs = specs.await;
                    cx.update(|_, cx| {
                        cx.open_url(&email_neopilot_url(&specs));
                    })
                    .log_err();
                })
                .detach();
            })
            .register_action(move |_, _: &OpenNeopilotRepo, _, cx| {
                cx.open_url(NEOPILOT_REPO_URL);
            });
    })
    .detach();
}
