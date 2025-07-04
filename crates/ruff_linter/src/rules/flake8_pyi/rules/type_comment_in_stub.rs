use std::sync::LazyLock;

use regex::Regex;

use ruff_macros::{ViolationMetadata, derive_message_formats};
use ruff_python_trivia::CommentRanges;

use crate::Locator;
use crate::Violation;
use crate::checkers::ast::LintContext;

/// ## What it does
/// Checks for the use of type comments (e.g., `x = 1  # type: int`) in stub
/// files.
///
/// ## Why is this bad?
/// Stub (`.pyi`) files should use type annotations directly, rather
/// than type comments, even if they're intended to support Python 2, since
/// stub files are not executed at runtime. The one exception is `# type: ignore`.
///
/// ## Example
/// ```pyi
/// x = 1  # type: int
/// ```
///
/// Use instead:
/// ```pyi
/// x: int = 1
/// ```
#[derive(ViolationMetadata)]
pub(crate) struct TypeCommentInStub;

impl Violation for TypeCommentInStub {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Don't use type comments in stub file".to_string()
    }
}

/// PYI033
pub(crate) fn type_comment_in_stub(
    context: &LintContext,
    locator: &Locator,
    comment_ranges: &CommentRanges,
) {
    for range in comment_ranges {
        let comment = locator.slice(range);

        if TYPE_COMMENT_REGEX.is_match(comment) && !TYPE_IGNORE_REGEX.is_match(comment) {
            context.report_diagnostic_if_enabled(TypeCommentInStub, range);
        }
    }
}

static TYPE_COMMENT_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^#\s*type:\s*([^#]+)(\s*#.*?)?$").unwrap());

static TYPE_IGNORE_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^#\s*type:\s*ignore([^#]+)?(\s*#.*?)?$").unwrap());
