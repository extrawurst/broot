use crate::app_context::AppContext;
use crate::conf::Conf;

/// build the markdown which will be displayed in the help page
///
pub fn build_markdown(con: &AppContext) -> String {
    let mut md = String::from(MD_HELP_INTRO);
    append_verbs_table(&mut md, con);
    append_config_info(&mut md, con);
    md.push_str(MD_HELP_LAUNCH_ARGUMENTS);
    md.push_str(MD_HELP_FLAGS);
    md
}


const MD_HELP_INTRO: &'static str = r#"
# Help

**broot** lets you explore directory trees and launch commands.
See https://dystroy.org/broot for a complete guide.

**broot** is best used when launched as `br`.
`<esc>` gets you back to the previous state.
Typing some letters searches the tree and selects the most relevant file.
To use a regular expression, use a slash eg `/j(ava|s)$`.

To execute a verb, type a space or `:` then start of its name or shortcut.

## Verbs

"#;

const MD_HELP_LAUNCH_ARGUMENTS: &'static str = r#"
## Launch Arguments

Some options can be set on launch:
* `-h` or `--hidden` : show hidden files
* `-f` or `--only-folders` : only show folders
* `-s` or `--sizes` : display sizes
 (for the complete list, run `broot --help`)
"#;

const MD_HELP_FLAGS: &'static str = r#"
## Flags

Flags are displayed at bottom right:
* `h:y` or `h:n` : whether hidden files are shown
* `gi:a`, `gi:y`, `gi:n` : whether gitignore is on `auto`, `yes` or `no`
 When gitignore is auto, .gitignore rules are respected if the displayed root is a git repository or in one.

"#;

fn append_verbs_table(md: &mut String, con: &AppContext) {
    md.push_str("|-:\n");
    md.push_str("|**name**|**shortcut**|**description**\n");
    md.push_str("|-:|:-:|:-\n");
    for verb in &con.verb_store.verbs {
        md.push_str(&format!(
            "|{}|{}|",
            verb.invocation.key,
            if let Some(sk) = &verb.shortcut {
                &sk
            } else {
                ""
            },
        ));
        if let Some(s) = &verb.description {
            md.push_str(&format!("{}\n", &s));
        } else {
            md.push_str(&format!("`{}`\n", &verb.execution));
        }
    }
    md.push_str("|:-|-|:-\n");
}

fn append_config_info(md: &mut String, _con: &AppContext) {
    md.push_str(&format!(
        " Verbs and skin can be configured in {:?}.\n",
        Conf::default_location()
    ));
}
