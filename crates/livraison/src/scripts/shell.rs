use std::collections::BTreeMap;

use indoc::{formatdoc, indoc};

use crate::scripts::doc::text;

use super::doc::Doc;

#[derive(Debug, Clone)]
pub struct PlatformMapping {
    /// Platform identifier as returned by `uname -ms`
    pub platform_id: String,
    /// Target name to use for this platform
    pub target: String,
}

#[derive(Debug)]
pub struct PlatformConfig {
    /// List of platform mappings to include
    pub mappings: Vec<PlatformMapping>,
    /// Default target to use if no platform matches (optional)
    pub default_target: Option<String>,
}

impl Default for PlatformConfig {
    fn default() -> Self {
        Self {
            mappings: vec![
                PlatformMapping {
                    platform_id: "Darwin x86_64".to_string(),
                    target: "darwin-x64".to_string(),
                },
                PlatformMapping {
                    platform_id: "Darwin arm64".to_string(),
                    target: "darwin-arm64".to_string(),
                },
                PlatformMapping {
                    platform_id: "Linux aarch64".to_string(),
                    target: "linux-arm64".to_string(),
                },
                PlatformMapping {
                    platform_id: "Linux arm64".to_string(),
                    target: "linux-arm64".to_string(),
                },
                PlatformMapping {
                    platform_id: "Linux x86_64".to_string(),
                    target: "linux-x64".to_string(),
                },
            ],
            default_target: Some("linux-x64".to_string()),
        }
    }
}

#[derive(Default, Debug)]
pub struct ShellScriptOptions {
    /// Product friendly name. Also the binary name if not provided
    pub name: String,

    /// Name of the binary
    pub bin_name: Option<String>,

    /// Filename template. Interpolate the following variables:
    /// - `{version}`: The version of the binary to download
    /// - `{target}`: The target platform for the binary
    /// - `{bin-name}`: The name of the binary
    ///
    /// Default to `{bin_name}-{target}.tar.gz`
    pub filename: Option<String>,
    /// URL template for downloading the binary. Interpolate the following variables:
    /// - `{version}`: The version of the binary to download
    /// - `{filename}`: The filename. [filename]
    /// - `{target}`: The target platform for the binary
    /// - `{bin-name}`: The name of the binary
    ///
    /// Example: "https://github.com/foo/bar/releases/{version}/{filename}"
    pub download_url: String,

    pub resolve_latest_version_url: Option<String>,
    pub platform_config: PlatformConfig,
}

impl ShellScriptOptions {
    pub fn get_bin_name(&self) -> &str {
        self.bin_name.as_deref().unwrap_or(&self.name)
    }
}

pub fn create_shell_script(options: ShellScriptOptions) -> String {
    let basic = text(&formatdoc! {r#"
        #!/usr/bin/env bash
        set -euo pipefail
        
        # Default values for args
        skip_shell=false # --skip-shell
        version="latest" # --version

        os=$(uname -s)
        platform=$(uname -ms)
        bin_name="{bin_name}"
        install_dir="$HOME/.{bin_name}"
    "#,
        bin_name = options.get_bin_name()
    });

    let mut script = vec![
        basic,
        Doc::Hardline,
        create_helpers(),
        Doc::Hardline,
        find_target(&options.platform_config),
        parse_args_fn(),
        get_filename_fn(&options),
        get_download_url_fn(&options),
    ];

    if let Some(url) = &options.resolve_latest_version_url {
        script.push(Doc::Hardline);
        script.push(find_latest_version_fn(url));
    }

    script.push(Doc::Hardline);
    script.push(download_function(&options));
    script.push(Doc::Hardline);
    script.push(check_dependencies_fn());
    script.push(Doc::Hardline);
    script.push(ensure_containing_dir_exists_fn());
    script.push(Doc::Hardline);
    script.push(setup_shell_fn(&options));
    script.push(Doc::Hardline);
    script.push(main_execution(&options));

    Doc::A(script).serialize()
}

fn create_helpers() -> Doc {
    text(indoc! {r#"
        # Reset
        Color_Off=''

        # Regular Colors
        Red=''
        Green=''
        Dim='' # White

        # Bold
        Bold_White=''
        Bold_Green=''

        if [[ -t 1 ]]; then
            # Reset
            Color_Off='\033[0m' # Text Reset

            # Regular Colors
            Red='\033[0;31m'   # Red
            Green='\033[0;32m' # Green
            Dim='\033[0;2m'    # White

            # Bold
            Bold_Green='\033[1;32m' # Bold Green
            Bold_White='\033[1m'    # Bold White
        fi

        error() {
            echo -e "${Red}error${Color_Off}:" "$@" >&2
            exit 1
        }

        info() {
            echo -e "${Dim}$@ ${Color_Off}"
        }

        success() {
            echo -e "${Green}$@ ${Color_Off}"
        }
    "#})
}

fn find_target(config: &PlatformConfig) -> Doc {
    let mut cases: Vec<Doc> = Vec::new();

    // Group mappings by target to handle multiple platform_ids mapping to the same target
    let mut target_groups: BTreeMap<String, Vec<String>> = BTreeMap::new();

    for mapping in &config.mappings {
        target_groups
            .entry(mapping.target.clone())
            .or_default()
            .push(mapping.platform_id.clone());
    }

    for (target, platform_ids) in target_groups {
        let conditions = platform_ids
            .iter()
            .map(|id| format!("'{}'", id))
            .collect::<Vec<_>>()
            .join(" | ");

        cases.push(Doc::A(vec![
            text(&format!("{conditions})")),
            Doc::Hardline,
            Doc::Indent(Box::new(Doc::A(vec![
                text(&format!("target={target}",)),
                Doc::Hardline,
                text(";;"),
            ]))),
        ]));
    }

    if let Some(default_target) = &config.default_target {
        cases.push(Doc::A(vec![
            text("*)"),
            Doc::Hardline,
            Doc::Indent(Box::new(Doc::A(vec![
                text(&format!("target={default_target}",)),
                Doc::Hardline,
                text(";;"),
            ]))),
        ]));
    }

    Doc::A(vec![
        text("case $platform in"),
        Doc::Hardline,
        Doc::Indent(Box::new(Doc::Join(cases, Box::new(Doc::Hardline)))),
        Doc::Hardline,
        text("esac"),
        Doc::Hardline,
        Doc::Hardline,
        rosetta_target_guard(config),
    ])
}

/// Add guard to make sure to install darwin aarch64
fn rosetta_target_guard(config: &PlatformConfig) -> Doc {
    let target = config
        .mappings
        .iter()
        .find(|x| x.platform_id == "Darwin arm64")
        .map(|x| x.target.clone());
    match target {
        None => text(""),
        Some(t) => text(&formatdoc! {r#"
            if [[ $platform = 'Darwin x86_64' ]]; then
                # Is this process running in Rosetta?
                # redirect stderr to devnull to avoid error message when not running in Rosetta
                if [[ $(sysctl -n sysctl.proc_translated 2> /dev/null) = 1 ]]; then
                    target={t}
                    info "Your shell is running in Rosetta 2. Downloading for $target instead"
                fi
            fi
        "#}),
    }
}

fn parse_args_fn() -> Doc {
    text(indoc! {r#"
            parse_args() {
                while [[ $# -gt 0 ]]; do
                    key="$1"

                    case $key in
                    # -d | --install-dir)
                    #   install_dir="$2"
                    #   shift # past argument
                    #   shift # past value
                    #   ;;
                    -s | --skip-shell)
                        SKIP_SHELL="true"
                        shift # past argument
                        ;;
                    --version)
                        version="$2"
                        shift # past release argument
                        shift # past release value
                        ;;
                    *)
                        echo "Unrecognized argument $key"
                        exit 1
                        ;;
                    esac
                done
            }
        "#})
}

fn find_latest_version_fn(latest_version_url: &str) -> Doc {
    text(&formatdoc! {r#"
        find_latest_version () {{
            curl "{latest_version_url}"
        }}
    "#})
}

fn get_filename_fn(options: &ShellScriptOptions) -> Doc {
    let filename = options
        .filename
        .clone()
        .unwrap_or("{bin_name}-$target.tar.gz".to_string())
        .replace("{version}", "$version")
        .replace("{bin_name}", options.get_bin_name())
        .replace("{target}", "$target");
    text(&formatdoc! {r#"
        get_filename() {{
            echo "{filename}"
        }}
    "#
    })
}

fn get_download_url_fn(options: &ShellScriptOptions) -> Doc {
    let interpolated_url = options
        .download_url
        .replace("{version}", "$version")
        .replace("{bin_name}", options.get_bin_name())
        .replace("{filename}", "$(get_filename)")
        .replace("{target}", "$target");

    text(&formatdoc! {r#"
        get_download_url() {{
            if [ "$version" = "latest" ]; then
                version=$(find_latest_version)
            fi

            echo "{interpolated_url}"
        }}
    "#,
    })
}

fn download_function(options: &ShellScriptOptions) -> Doc {
    text(&formatdoc! {r#"
        download_{bin_name}() {{
          URL=$(get_download_url)
          info "Downloading {name} from $URL"

          download_dir=$(mktemp -d)
          filename=$(get_filename)

          echo "Downloading $URL..."
          bin_dir="$install_dir/bin"

          compressed_file_path="$download_dir/$filename"
          if ! curl --progress-bar --fail -L "$URL" -o "$compressed_file_path"; then
            error "Download failed.  Check that the release/filename are correct."
            exit 1
          fi

          extract_location="$download_dir/extracted"
          mkdir $extract_location
          tar -zxvf "$compressed_file_path" -C "$extract_location"/
          rm "$compressed_file_path"
          chmod +x "$extract_location/$bin_name"

          # Move to install directory
          mkdir -p "$bin_dir" &> /dev/null
          mv "$extract_location/$bin_name" "$bin_dir/$bin_name"
          success "{name} was installed successfully to $Bold_Green$("$install_dir")"
        }}
    "#,
        bin_name = options.get_bin_name(),
        name = options.name,
    })
}

fn check_dependencies_fn() -> Doc {
    text(indoc! {r#"
        check_dependencies() {
          should_exit="false"
          info "Checking dependencies for the installation script..."

          info "Checking availability of curl... "
          if hash curl 2> /dev/null; then
            info "OK!"
          else
            error "curl is required to install"
            should_exit="true"
          fi

          if [ "$should_exit" = "true" ]; then
            info "Not installing due to missing dependencies."
            exit 1
          fi
        }
    "#})
}

fn ensure_containing_dir_exists_fn() -> Doc {
    text(indoc! {r#"
        ensure_containing_dir_exists() {
          local CONTAINING_DIR
          CONTAINING_DIR="$(dirname "$1")"
          if [ ! -d "$CONTAINING_DIR" ]; then
            echo " >> Creating directory $CONTAINING_DIR"
            mkdir -p "$CONTAINING_DIR"
          fi
        }
    "#})
}

fn setup_shell_fn(options: &ShellScriptOptions) -> Doc {
    text(&formatdoc! {r#"
        setup_shell() {{
          CURRENT_SHELL="$(basename "$SHELL")"

          if [ "$CURRENT_SHELL" = "zsh" ]; then
            CONF_FILE=${{ZDOTDIR:-$HOME}}/.zshrc
            ensure_containing_dir_exists "$CONF_FILE"
            echo "Installing for Zsh. Appending the following to $CONF_FILE:"
            {{
              echo ''
              echo '# {name}'
              echo '{name_upper}_PATH="'"$bin_dir"'"'
              echo 'if [ -d "${name_upper}_PATH" ]; then'
              echo '  export PATH="'$bin_dir':$PATH"'
              echo 'fi'
            }} | tee -a "$CONF_FILE"

          elif [ "$CURRENT_SHELL" = "fish" ]; then
            CONF_FILE=$HOME/.config/fish/conf.d/{bin_name}.fish
            ensure_containing_dir_exists "$CONF_FILE"
            echo "Installing for Fish. Appending the following to $CONF_FILE:"
            {{
              echo ''
              echo '# {name}'
              echo 'set {name_upper}_PATH "'"$bin_dir"'"'
              echo 'if [ -d "${name_upper}_PATH" ]'
              echo '  set PATH "${name_upper}_PATH" $PATH'
              echo 'end'
            }} | tee -a "$CONF_FILE"

          elif [ "$CURRENT_SHELL" = "bash" ]; then
            if [ "$os" = "Darwin" ]; then
              CONF_FILE=$HOME/.profile
            else
              CONF_FILE=$HOME/.bashrc
            fi
            ensure_containing_dir_exists "$CONF_FILE"
            echo "Installing for Bash. Appending the following to $CONF_FILE:"
            {{
              echo ''
              echo '# {name}'
              echo '{name_upper}_PATH="'"$bin_dir"'"'
              echo 'if [ -d "${name_upper}_PATH" ]; then'
              echo '  export PATH="${name_upper}_PATH:$PATH"'
              echo 'fi'
            }} | tee -a "$CONF_FILE"

          else
            error "Could not infer shell type. Please set up manually."
            exit 1
          fi

          info ""
          info "In order to apply the changes, open a new terminal or run the following command:"
          info ""
          info "  source $CONF_FILE"
        }}
    "#,
        bin_name = options.get_bin_name(),
        name = options.name,
        name_upper = options.name.to_uppercase()
    })
}

fn main_execution(options: &ShellScriptOptions) -> Doc {
    text(&formatdoc! {r#"
        parse_args "$@"
        check_dependencies
        download_{bin_name}
        if [ "$skip_shell" != "true" ]; then
          setup_shell
        fi
    "#,
        bin_name = options.get_bin_name()
    })
}

#[cfg(test)]
mod tests {
    use crate::scripts::shell::{ShellScriptOptions, create_shell_script};

    #[test]
    fn test_create_basic() {
        let script = create_shell_script(ShellScriptOptions {
            name: "test".to_string(),
            download_url: "https://example.com/{version}/{filename}".to_string(),
            ..Default::default()
        });

        insta::assert_binary_snapshot!("basic.sh", script.into());
    }
}
