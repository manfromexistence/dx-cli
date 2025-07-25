use super::{
    AnalyzerVisitorBuilder, CodeActionsParams, EnabledForPath, ExtensionHandler, FixAllParams,
    LintParams, LintResults, ParseResult, ProcessLint, SearchCapabilities, is_diagnostic_error,
    search,
};
use crate::WorkspaceError;
use crate::configuration::to_analyzer_rules;
use crate::file_handlers::DebugCapabilities;
use crate::file_handlers::{
    AnalyzerCapabilities, Capabilities, FormatterCapabilities, ParserCapabilities,
};
use crate::settings::{
    FormatSettings, LanguageListSettings, LanguageSettings, OverrideSettings, ServiceLanguage,
    Settings, check_feature_activity, check_override_feature_activity,
};
use crate::workspace::{
    CodeAction, DocumentFileSource, FixAction, FixFileMode, FixFileResult, GetSyntaxTreeResult,
    PullActionsResult,
};
use biome_analyze::options::PreferredQuote;
use biome_analyze::{
    AnalysisFilter, AnalyzerConfiguration, AnalyzerOptions, ControlFlow, Never, RuleError,
};
use biome_configuration::css::{
    CssAllowWrongLineCommentsEnabled, CssAssistConfiguration, CssAssistEnabled,
    CssFormatterConfiguration, CssFormatterEnabled, CssLinterConfiguration, CssLinterEnabled,
    CssModulesEnabled, CssParserConfiguration,
};
use biome_css_analyze::analyze;
use biome_css_formatter::context::CssFormatOptions;
use biome_css_formatter::format_node;
use biome_css_parser::CssParserOptions;
use biome_css_syntax::{CssLanguage, CssRoot, CssSyntaxNode};
use biome_diagnostics::Applicability;
use biome_formatter::{
    FormatError, IndentStyle, IndentWidth, LineEnding, LineWidth, Printed, QuoteStyle,
};
use biome_fs::BiomePath;
use biome_parser::AnyParse;
use biome_rowan::{AstNode, NodeCache};
use biome_rowan::{TextRange, TextSize, TokenAtOffset};
use camino::Utf8Path;
use std::borrow::Cow;
use tracing::{debug_span, error, info, trace_span};

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct CssFormatterSettings {
    pub line_ending: Option<LineEnding>,
    pub line_width: Option<LineWidth>,
    pub indent_width: Option<IndentWidth>,
    pub indent_style: Option<IndentStyle>,
    pub quote_style: Option<QuoteStyle>,
    pub enabled: Option<CssFormatterEnabled>,
}

impl From<CssFormatterConfiguration> for CssFormatterSettings {
    fn from(configuration: CssFormatterConfiguration) -> Self {
        Self {
            enabled: configuration.enabled,
            line_width: configuration.line_width,
            indent_width: configuration.indent_width,
            indent_style: configuration.indent_style,
            quote_style: configuration.quote_style,
            line_ending: configuration.line_ending,
        }
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct CssLinterSettings {
    pub enabled: Option<CssLinterEnabled>,
    pub suppression_reason: Option<String>,
}

impl From<CssLinterConfiguration> for CssLinterSettings {
    fn from(value: CssLinterConfiguration) -> Self {
        Self {
            enabled: value.enabled,
            suppression_reason: None,
        }
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct CssAssistSettings {
    pub enabled: Option<CssAssistEnabled>,
}

impl From<CssAssistConfiguration> for CssAssistSettings {
    fn from(configuration: CssAssistConfiguration) -> Self {
        Self {
            enabled: configuration.enabled,
        }
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct CssParserSettings {
    pub allow_wrong_line_comments: Option<CssAllowWrongLineCommentsEnabled>,
    pub css_modules_enabled: Option<CssModulesEnabled>,
}

impl From<CssParserConfiguration> for CssParserSettings {
    fn from(configuration: CssParserConfiguration) -> Self {
        Self {
            allow_wrong_line_comments: configuration.allow_wrong_line_comments,
            css_modules_enabled: configuration.css_modules,
        }
    }
}

impl CssFormatterSettings {
    pub fn is_enabled(&self) -> bool {
        self.enabled.unwrap_or_default().into()
    }
}

impl CssParserSettings {
    pub fn css_modules_enabled(&self) -> bool {
        self.css_modules_enabled.unwrap_or_default().into()
    }

    pub fn allow_wrong_line_comments(&self) -> bool {
        self.allow_wrong_line_comments.unwrap_or_default().into()
    }
}

impl ServiceLanguage for CssLanguage {
    type FormatterSettings = CssFormatterSettings;
    type LinterSettings = CssLinterSettings;
    type FormatOptions = CssFormatOptions;
    type ParserSettings = CssParserSettings;
    type AssistSettings = CssAssistSettings;
    type EnvironmentSettings = ();

    fn lookup_settings(language: &LanguageListSettings) -> &LanguageSettings<Self> {
        &language.css
    }

    fn resolve_format_options(
        global: &FormatSettings,
        overrides: &OverrideSettings,
        language: &Self::FormatterSettings,
        path: &BiomePath,
        document_file_source: &DocumentFileSource,
    ) -> Self::FormatOptions {
        let indent_style = language
            .indent_style
            .or(global.indent_style)
            .unwrap_or_default();
        let line_width = language
            .line_width
            .or(global.line_width)
            .unwrap_or_default();
        let indent_width = language
            .indent_width
            .or(global.indent_width)
            .unwrap_or_default();

        let line_ending = language
            .line_ending
            .or(global.line_ending)
            .unwrap_or_default();

        let mut options = CssFormatOptions::new(
            document_file_source
                .to_css_file_source()
                .unwrap_or_default(),
        )
        .with_indent_style(indent_style)
        .with_indent_width(indent_width)
        .with_line_width(line_width)
        .with_line_ending(line_ending)
        .with_quote_style(language.quote_style.unwrap_or_default());

        overrides.apply_override_css_format_options(path, &mut options);

        options
    }

    fn resolve_analyzer_options(
        global: &Settings,
        _language: &Self::LinterSettings,
        _environment: Option<&Self::EnvironmentSettings>,
        file_path: &BiomePath,
        _file_source: &DocumentFileSource,
        suppression_reason: Option<&str>,
    ) -> AnalyzerOptions {
        let preferred_quote = global
            .languages
            .css
            .formatter
            .quote_style
            .map(|quote_style: QuoteStyle| {
                if quote_style == QuoteStyle::Single {
                    PreferredQuote::Single
                } else {
                    PreferredQuote::Double
                }
            })
            .unwrap_or_default();

        let configuration = AnalyzerConfiguration::default()
            .with_rules(to_analyzer_rules(global, file_path.as_path()))
            .with_preferred_quote(preferred_quote)
            .with_css_modules(
                global
                    .languages
                    .css
                    .parser
                    .css_modules_enabled
                    .is_some_and(|css_modules_enabled| css_modules_enabled.into()),
            );

        AnalyzerOptions::default()
            .with_file_path(file_path.as_path())
            .with_configuration(configuration)
            .with_suppression_reason(suppression_reason)
    }

    fn formatter_enabled_for_file_path(settings: &Settings, path: &Utf8Path) -> bool {
        let overrides_activity =
            settings
                .override_settings
                .patterns
                .iter()
                .rev()
                .find_map(|pattern| {
                    check_override_feature_activity(
                        pattern.languages.css.formatter.enabled,
                        pattern.formatter.enabled,
                    )
                    .filter(|_| {
                        // Then check whether the path satisfies
                        pattern.is_file_included(path)
                    })
                });

        overrides_activity
            .or(check_feature_activity(
                settings.languages.css.formatter.enabled,
                settings.formatter.enabled,
            ))
            .unwrap_or_default()
            .into()
    }

    fn assist_enabled_for_file_path(settings: &Settings, path: &Utf8Path) -> bool {
        let overrides_activity =
            settings
                .override_settings
                .patterns
                .iter()
                .rev()
                .find_map(|pattern| {
                    check_override_feature_activity(
                        pattern.languages.css.assist.enabled,
                        pattern.assist.enabled,
                    )
                    .filter(|_| {
                        // Then check whether the path satisfies
                        pattern.is_file_included(path)
                    })
                });

        overrides_activity
            .or(check_feature_activity(
                settings.languages.css.assist.enabled,
                settings.assist.enabled,
            ))
            .unwrap_or_default()
            .into()
    }

    fn linter_enabled_for_file_path(settings: &Settings, path: &Utf8Path) -> bool {
        let overrides_activity =
            settings
                .override_settings
                .patterns
                .iter()
                .rev()
                .find_map(|pattern| {
                    check_override_feature_activity(
                        pattern.languages.css.linter.enabled,
                        pattern.linter.enabled,
                    )
                    .filter(|_| {
                        // Then check whether the path satisfies
                        pattern.is_file_included(path)
                    })
                });

        overrides_activity
            .or(check_feature_activity(
                settings.languages.css.linter.enabled,
                settings.linter.enabled,
            ))
            .unwrap_or_default()
            .into()
    }

    fn resolve_environment(_settings: &Settings) -> Option<&Self::EnvironmentSettings> {
        None
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub(crate) struct CssFileHandler;

impl ExtensionHandler for CssFileHandler {
    fn capabilities(&self) -> Capabilities {
        Capabilities {
            parser: ParserCapabilities { parse: Some(parse) },
            debug: DebugCapabilities {
                debug_syntax_tree: Some(debug_syntax_tree),
                debug_control_flow: None,
                debug_formatter_ir: Some(debug_formatter_ir),
                debug_type_info: None,
                debug_registered_types: None,
                debug_semantic_model: None,
            },
            analyzer: AnalyzerCapabilities {
                lint: Some(lint),
                code_actions: Some(code_actions),
                rename: None,
                fix_all: Some(fix_all),
            },
            formatter: FormatterCapabilities {
                format: Some(format),
                format_range: Some(format_range),
                format_on_type: Some(format_on_type),
            },
            search: SearchCapabilities {
                search: Some(search),
            },
            enabled_for_path: EnabledForPath {
                formatter: Some(formatter_enabled),
                linter: Some(linter_enabled),
                assist: Some(assist_enabled),
                search: Some(search_enabled),
            },
        }
    }
}

fn formatter_enabled(path: &Utf8Path, settings: &Settings) -> bool {
    settings.formatter_enabled_for_file_path::<CssLanguage>(path)
}

fn linter_enabled(path: &Utf8Path, settings: &Settings) -> bool {
    settings.linter_enabled_for_file_path::<CssLanguage>(path)
}

fn assist_enabled(path: &Utf8Path, settings: &Settings) -> bool {
    settings.assist_enabled_for_file_path::<CssLanguage>(path)
}

fn search_enabled(_path: &Utf8Path, _settings: &Settings) -> bool {
    true
}

fn parse(
    biome_path: &BiomePath,
    _file_source: DocumentFileSource,
    text: &str,
    settings: &Settings,
    cache: &mut NodeCache,
) -> ParseResult {
    let mut options = CssParserOptions {
        allow_wrong_line_comments: settings
            .languages
            .css
            .parser
            .allow_wrong_line_comments
            .unwrap_or_default()
            .into(),
        css_modules: settings
            .languages
            .css
            .parser
            .css_modules_enabled
            .unwrap_or_default()
            .into(),
        grit_metavariables: false,
    };

    settings
        .override_settings
        .apply_override_css_parser_options(biome_path, &mut options);

    let parse = biome_css_parser::parse_css_with_cache(text, cache, options);
    ParseResult {
        any_parse: parse.into(),
        language: None,
    }
}

fn debug_syntax_tree(_rome_path: &BiomePath, parse: AnyParse) -> GetSyntaxTreeResult {
    let syntax: CssSyntaxNode = parse.syntax();
    let tree: CssRoot = parse.tree();
    GetSyntaxTreeResult {
        cst: format!("{syntax:#?}"),
        ast: format!("{tree:#?}"),
    }
}

fn debug_formatter_ir(
    biome_path: &BiomePath,
    document_file_source: &DocumentFileSource,
    parse: AnyParse,
    settings: &Settings,
) -> Result<String, WorkspaceError> {
    let options = settings.format_options::<CssLanguage>(biome_path, document_file_source);

    let tree = parse.syntax();
    let formatted = format_node(options, &tree)?;

    let root_element = formatted.into_document();
    Ok(root_element.to_string())
}

#[tracing::instrument(level = "debug", skip(parse))]
fn format(
    biome_path: &BiomePath,
    document_file_source: &DocumentFileSource,
    parse: AnyParse,
    settings: &Settings,
) -> Result<Printed, WorkspaceError> {
    let options = settings.format_options::<CssLanguage>(biome_path, document_file_source);

    let tree = parse.syntax();
    let formatted = format_node(options, &tree)?;

    match formatted.print() {
        Ok(printed) => Ok(printed),
        Err(error) => Err(WorkspaceError::FormatError(error.into())),
    }
}

fn format_range(
    biome_path: &BiomePath,
    document_file_source: &DocumentFileSource,
    parse: AnyParse,
    settings: &Settings,
    range: TextRange,
) -> Result<Printed, WorkspaceError> {
    let options = settings.format_options::<CssLanguage>(biome_path, document_file_source);

    let tree = parse.syntax();
    let printed = biome_css_formatter::format_range(options, &tree, range)?;
    Ok(printed)
}

fn format_on_type(
    biome_path: &BiomePath,
    document_file_source: &DocumentFileSource,
    parse: AnyParse,
    settings: &Settings,
    offset: TextSize,
) -> Result<Printed, WorkspaceError> {
    let options = settings.format_options::<CssLanguage>(biome_path, document_file_source);

    let tree = parse.syntax();

    let range = tree.text_range_with_trivia();
    if offset < range.start() || offset > range.end() {
        return Err(WorkspaceError::FormatError(FormatError::RangeError {
            input: TextRange::at(offset, TextSize::from(0)),
            tree: range,
        }));
    }

    let token = match tree.token_at_offset(offset) {
        // File is empty, do nothing
        TokenAtOffset::None => panic!("empty file"),
        TokenAtOffset::Single(token) => token,
        // The cursor should be right after the closing character that was just typed,
        // select the previous token as the correct one
        TokenAtOffset::Between(token, _) => token,
    };

    let root_node = match token.parent() {
        Some(node) => node,
        None => panic!("found a token with no parent"),
    };

    let printed = biome_css_formatter::format_sub_tree(options, &root_node)?;
    Ok(printed)
}

fn lint(params: LintParams) -> LintResults {
    let _ =
        debug_span!("Linting CSS file", path =? params.path, language =? params.language).entered();
    let settings = &params.settings;
    let analyzer_options = settings.analyzer_options::<CssLanguage>(
        params.path,
        &params.language,
        params.suppression_reason.as_deref(),
    );
    let tree = params.parse.tree();

    let (enabled_rules, disabled_rules, analyzer_options) =
        AnalyzerVisitorBuilder::new(settings, analyzer_options)
            .with_only(&params.only)
            .with_skip(&params.skip)
            .with_path(params.path.as_path())
            .with_enabled_rules(&params.enabled_rules)
            .with_project_layout(params.project_layout.clone())
            .finish();

    let filter = AnalysisFilter {
        categories: params.categories,
        enabled_rules: Some(enabled_rules.as_slice()),
        disabled_rules: &disabled_rules,
        range: None,
    };

    let mut process_lint = ProcessLint::new(&params);

    let (_, analyze_diagnostics) = analyze(
        &tree,
        filter,
        &analyzer_options,
        &params.plugins,
        |signal| process_lint.process_signal(signal),
    );

    process_lint.into_result(params.parse.into_diagnostics(), analyze_diagnostics)
}

#[tracing::instrument(level = "debug", skip(params))]
pub(crate) fn code_actions(params: CodeActionsParams) -> PullActionsResult {
    let CodeActionsParams {
        parse,
        range,
        settings,
        path,
        module_graph: _,
        project_layout,
        language,
        only,
        skip,
        enabled_rules: rules,
        suppression_reason,
        plugins,
        categories,
    } = params;
    let _ = debug_span!("Code actions CSS", range =? range, path =? path).entered();
    let tree = parse.tree();
    let _ = trace_span!("Parsed file", tree =? tree).entered();
    let Some(_) = language.to_css_file_source() else {
        error!("Could not determine the file source of the file");
        return PullActionsResult {
            actions: Vec::new(),
        };
    };

    let analyzer_options =
        settings.analyzer_options::<CssLanguage>(path, &language, suppression_reason.as_deref());
    let mut actions = Vec::new();
    let (enabled_rules, disabled_rules, analyzer_options) =
        AnalyzerVisitorBuilder::new(settings, analyzer_options)
            .with_only(&only)
            .with_skip(&skip)
            .with_path(path.as_path())
            .with_enabled_rules(&rules)
            .with_project_layout(project_layout)
            .finish();

    let filter = AnalysisFilter {
        categories,
        enabled_rules: Some(enabled_rules.as_slice()),
        disabled_rules: &disabled_rules,
        range,
    };

    info!("CSS runs the analyzer");

    analyze(&tree, filter, &analyzer_options, &plugins, |signal| {
        actions.extend(signal.actions().into_code_action_iter().map(|item| {
            CodeAction {
                category: item.category.clone(),
                rule_name: item
                    .rule_name
                    .map(|(group, name)| (Cow::Borrowed(group), Cow::Borrowed(name))),
                suggestion: item.suggestion,
            }
        }));

        ControlFlow::<Never>::Continue(())
    });

    PullActionsResult { actions }
}

/// Applies all the safe fixes to the given syntax tree.
pub(crate) fn fix_all(params: FixAllParams) -> Result<FixFileResult, WorkspaceError> {
    let mut tree: CssRoot = params.parse.tree();

    // Compute final rules (taking `overrides` into account)
    let rules = params.settings.as_linter_rules(params.biome_path.as_path());
    let analyzer_options = params.settings.analyzer_options::<CssLanguage>(
        params.biome_path,
        &params.document_file_source,
        params.suppression_reason.as_deref(),
    );
    let (enabled_rules, disabled_rules, analyzer_options) =
        AnalyzerVisitorBuilder::new(params.settings, analyzer_options)
            .with_only(&params.only)
            .with_skip(&params.skip)
            .with_path(params.biome_path.as_path())
            .with_enabled_rules(&params.enabled_rules)
            .with_project_layout(params.project_layout)
            .finish();

    let filter = AnalysisFilter {
        categories: params.rule_categories,
        enabled_rules: Some(enabled_rules.as_slice()),
        disabled_rules: &disabled_rules,
        range: None,
    };

    let mut actions = Vec::new();
    let mut skipped_suggested_fixes = 0;
    let mut errors: u16 = 0;

    loop {
        let (action, _) = analyze(
            &tree,
            filter,
            &analyzer_options,
            &params.plugins,
            |signal| {
                let current_diagnostic = signal.diagnostic();

                if let Some(diagnostic) = current_diagnostic.as_ref() {
                    if is_diagnostic_error(diagnostic, rules.as_deref()) {
                        errors += 1;
                    }
                }

                for action in signal.actions() {
                    // suppression actions should not be part of the fixes (safe or suggested)
                    if action.is_suppression() {
                        continue;
                    }

                    match params.fix_file_mode {
                        FixFileMode::SafeFixes => {
                            if action.applicability == Applicability::MaybeIncorrect {
                                skipped_suggested_fixes += 1;
                            }
                            if action.applicability == Applicability::Always {
                                errors = errors.saturating_sub(1);
                                return ControlFlow::Break(action);
                            }
                        }
                        FixFileMode::SafeAndUnsafeFixes => {
                            if matches!(
                                action.applicability,
                                Applicability::Always | Applicability::MaybeIncorrect
                            ) {
                                errors = errors.saturating_sub(1);
                                return ControlFlow::Break(action);
                            }
                        }
                        FixFileMode::ApplySuppressions => {
                            // TODO: to implement
                        }
                    }
                }

                ControlFlow::Continue(())
            },
        );

        match action {
            Some(action) => {
                if let (root, Some((range, _))) =
                    action.mutation.commit_with_text_range_and_edit(true)
                {
                    tree = match CssRoot::cast(root) {
                        Some(tree) => tree,
                        None => {
                            return Err(WorkspaceError::RuleError(
                                RuleError::ReplacedRootWithNonRootError {
                                    rule_name: action.rule_name.map(|(group, rule)| {
                                        (Cow::Borrowed(group), Cow::Borrowed(rule))
                                    }),
                                },
                            ));
                        }
                    };
                    actions.push(FixAction {
                        rule_name: action
                            .rule_name
                            .map(|(group, rule)| (Cow::Borrowed(group), Cow::Borrowed(rule))),
                        range,
                    });
                }
            }
            None => {
                let code = if params.should_format {
                    format_node(
                        params.settings.format_options::<CssLanguage>(
                            params.biome_path,
                            &params.document_file_source,
                        ),
                        tree.syntax(),
                    )?
                    .print()?
                    .into_code()
                } else {
                    tree.syntax().to_string()
                };
                return Ok(FixFileResult {
                    code,
                    skipped_suggested_fixes,
                    actions,
                    errors: errors.into(),
                });
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use biome_css_syntax::CssFileSource;

    #[test]
    fn inherit_global_format_settings() {
        let format_options = CssLanguage::resolve_format_options(
            &FormatSettings::default(),
            &OverrideSettings::default(),
            &CssFormatterSettings::default(),
            &BiomePath::new(""),
            &DocumentFileSource::Css(CssFileSource::css()),
        );
        assert_eq!(
            format_options,
            CssFormatOptions::default()
                .with_indent_style(IndentStyle::default())
                .with_indent_width(IndentWidth::default())
                .with_line_ending(LineEnding::default())
                .with_line_width(LineWidth::default())
        );
    }
}
