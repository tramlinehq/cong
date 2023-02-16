use askama::Template;
use serde_derive::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter, EnumString};

#[derive(Debug, Serialize, Deserialize)]
pub struct State {
    pub platform: Platform,
    pub sdk: Sdk,
    pub build_type: BuildType,
    pub custom_inputs: CustomInputs,
    pub code_template: Option<String>,
    pub info_template: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomInputs {
    pub build_variant_name: Option<String>,
    pub build_variant_path: Option<String>,
    pub publishing_format: PublishingFormat,
    pub show_versions: bool,
}

impl State {
    pub fn clear_text(&mut self) {
        self.info_template = None;
        self.code_template = Some(String::new());
    }

    pub fn gen_templates(&mut self) {
        let (code_template, info_template) = match (self.platform, self.sdk, self.build_type) {
            (Platform::GitHub, Sdk::Native, BuildType::Signed) => {
                let code_template = GithubNativeSigned {
                    title: "Android release build",
                    publishing_format: &self.custom_inputs.publishing_format.to_owned(),
                    show_versions: &self.custom_inputs.show_versions.to_owned(),
                    build_variant_name: &self
                        .custom_inputs
                        .build_variant_name
                        .to_owned()
                        .unwrap_or_default(),
                    build_variant_path: &self
                        .custom_inputs
                        .build_variant_path
                        .to_owned()
                        .unwrap_or_default(),
                };

                let info_template = GithubNativeSignedInfo {
                    show_versions: &self.custom_inputs.show_versions.to_owned(),
                };

                (
                    Some(code_template.render().unwrap()),
                    Some(info_template.render().unwrap()),
                )
            }

            (Platform::GitHub, Sdk::Flutter, BuildType::Signed) => {
                let code_template = GithubFlutterSigned {
                    title: "Flutter Android release build",
                    publishing_format: &self.custom_inputs.publishing_format.to_owned(),
                    show_versions: &self.custom_inputs.show_versions.to_owned(),
                    build_variant_path: &self
                        .custom_inputs
                        .build_variant_path
                        .to_owned()
                        .unwrap_or_default(),
                };

                let info_template = GithubFlutterSignedInfo {
                    show_versions: &self.custom_inputs.show_versions.to_owned(),
                };

                (
                    Some(code_template.render().unwrap()),
                    Some(info_template.render().unwrap()),
                )
            }

            (Platform::GitHub, Sdk::ReactNative, BuildType::Signed) => {
                let code_template = GithubReactNativeSigned {
                    title: "React Native Android release build",
                    publishing_format: &self.custom_inputs.publishing_format.to_owned(),
                    show_versions: &self.custom_inputs.show_versions.to_owned(),
                    build_variant_name: &self
                        .custom_inputs
                        .build_variant_name
                        .to_owned()
                        .unwrap_or_default(),
                    build_variant_path: &self
                        .custom_inputs
                        .build_variant_path
                        .to_owned()
                        .unwrap_or_default(),
                };

                let info_template = GithubReactNativeSignedInfo {
                    show_versions: &self.custom_inputs.show_versions.to_owned(),
                };

                (
                    Some(code_template.render().unwrap()),
                    Some(info_template.render().unwrap()),
                )
            }

            (Platform::GitHub, Sdk::Native, BuildType::Unsigned) => {
                let code_template = GithubNativeUnsigned {
                    title: "Android debug build",
                    publishing_format: &self.custom_inputs.publishing_format.to_owned(),
                    show_versions: &self.custom_inputs.show_versions.to_owned(),
                    build_variant_name: &self
                        .custom_inputs
                        .build_variant_name
                        .to_owned()
                        .unwrap_or_default(),
                    build_variant_path: &self
                        .custom_inputs
                        .build_variant_path
                        .to_owned()
                        .unwrap_or_default(),
                };

                let info_template = GithubNativeUnsignedInfo {
                    show_versions: &self.custom_inputs.show_versions.to_owned(),
                };

                (
                    Some(code_template.render().unwrap()),
                    Some(info_template.render().unwrap()),
                )
            }

            (Platform::GitHub, Sdk::Flutter, BuildType::Unsigned) => {
                let code_template = GithubFlutterUnsigned {
                    title: "Flutter Android debug build",
                    publishing_format: &self.custom_inputs.publishing_format.to_owned(),
                    show_versions: &self.custom_inputs.show_versions.to_owned(),
                    build_variant_path: &self
                        .custom_inputs
                        .build_variant_path
                        .to_owned()
                        .unwrap_or_default(),
                };

                let info_template = GithubFlutterUnsignedInfo {
                    show_versions: &self.custom_inputs.show_versions.to_owned(),
                };

                (
                    Some(code_template.render().unwrap()),
                    Some(info_template.render().unwrap()),
                )
            }

            (Platform::GitHub, Sdk::ReactNative, BuildType::Unsigned) => {
                let code_template = GithubReactNativeUnsigned {
                    title: "React Native Android debug build",
                    publishing_format: &self.custom_inputs.publishing_format.to_owned(),
                    show_versions: &self.custom_inputs.show_versions.to_owned(),
                    build_variant_name: &self
                        .custom_inputs
                        .build_variant_name
                        .to_owned()
                        .unwrap_or_default(),
                    build_variant_path: &self
                        .custom_inputs
                        .build_variant_path
                        .to_owned()
                        .unwrap_or_default(),
                };

                (Some(code_template.render().unwrap()), None)
            }
        };

        self.code_template = code_template;
        self.info_template = info_template
    }
}

#[derive(
    Clone, Copy, Debug, EnumIter, EnumString, Display, PartialEq, Serialize, Deserialize, Eq,
)]
pub enum Platform {
    #[strum(serialize = "GitHub Actions")]
    GitHub,
}

#[derive(
    Clone, Copy, Debug, EnumIter, EnumString, Display, PartialEq, Serialize, Deserialize, Eq,
)]
pub enum Sdk {
    #[strum(serialize = "Native App")]
    Native,
    #[strum(serialize = "Flutter")]
    Flutter,
    #[strum(serialize = "React Native")]
    ReactNative,
}

#[derive(
    Clone, Copy, Debug, EnumIter, EnumString, Display, PartialEq, Serialize, Deserialize, Eq,
)]
pub enum BuildType {
    #[strum(serialize = "Debug (unsigned)")]
    Unsigned,
    #[strum(serialize = "Release (signed)")]
    Signed,
}

#[derive(
    Clone, Copy, Debug, EnumIter, EnumString, Display, PartialEq, Serialize, Deserialize, Eq,
)]
pub enum PublishingFormat {
    #[strum(serialize = "APK")]
    Apk,
    #[strum(serialize = "AAB")]
    Aab,
}

#[derive(Template)]
#[template(path = "workflows/github-native-signed")]
struct GithubNativeSigned<'a> {
    title: &'a str,
    publishing_format: &'a PublishingFormat,
    show_versions: &'a bool,
    build_variant_name: &'a str,
    build_variant_path: &'a str,
}

#[derive(Template)]
#[template(path = "workflows/github-flutter-signed")]
struct GithubFlutterSigned<'a> {
    title: &'a str,
    publishing_format: &'a PublishingFormat,
    show_versions: &'a bool,
    build_variant_path: &'a str,
}

#[derive(Template)]
#[template(path = "workflows/github-react-native-signed")]
struct GithubReactNativeSigned<'a> {
    title: &'a str,
    publishing_format: &'a PublishingFormat,
    show_versions: &'a bool,
    build_variant_name: &'a str,
    build_variant_path: &'a str,
}

#[derive(Template)]
#[template(path = "workflows/github-native-unsigned")]
struct GithubNativeUnsigned<'a> {
    title: &'a str,
    publishing_format: &'a PublishingFormat,
    show_versions: &'a bool,
    build_variant_name: &'a str,
    build_variant_path: &'a str,
}

#[derive(Template)]
#[template(path = "workflows/github-flutter-unsigned")]
struct GithubFlutterUnsigned<'a> {
    title: &'a str,
    publishing_format: &'a PublishingFormat,
    show_versions: &'a bool,
    build_variant_path: &'a str,
}

#[derive(Template)]
#[template(path = "workflows/github-react-native-unsigned")]
struct GithubReactNativeUnsigned<'a> {
    title: &'a str,
    publishing_format: &'a PublishingFormat,
    show_versions: &'a bool,
    build_variant_name: &'a str,
    build_variant_path: &'a str,
}

#[derive(Template)]
#[template(path = "info/github-native-signed")]
struct GithubNativeSignedInfo<'a> {
    show_versions: &'a bool,
}

#[derive(Template)]
#[template(path = "info/github-native-unsigned")]
struct GithubNativeUnsignedInfo<'a> {
    show_versions: &'a bool,
}

#[derive(Template)]
#[template(path = "info/github-flutter-signed")]
struct GithubFlutterSignedInfo<'a> {
    show_versions: &'a bool,
}

#[derive(Template)]
#[template(path = "info/github-flutter-unsigned")]
struct GithubFlutterUnsignedInfo<'a> {
    show_versions: &'a bool,
}

#[derive(Template)]
#[template(path = "info/github-react-native-signed")]
struct GithubReactNativeSignedInfo<'a> {
    show_versions: &'a bool,
}
