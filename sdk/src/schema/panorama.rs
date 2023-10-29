// scope panorama.dll
// 2023-10-29 23:17:16.851693400 UTC

pub enum ELayoutNodeType {
    Root,
    Styles,
    ScriptBody,
    Scripts,
    Snippets,
    Include,
    Snippet,
    Panel,
    PanelAttribute,
    PanelAttributeValue,
    ReferenceContent,
    ReferenceCompiled,
    ReferencePassthrough,
}

pub enum EStyleNodeType {
    Root,
    Expression,
    Property,
    Define,
    Import,
    Keyframes,
    KeyframeSelector,
    StyleSelector,
    Whitespace,
    ExpressionText,
    ExpressionUrl,
    ExpressionConcat,
    ReferenceContent,
    ReferenceCompiled,
    ReferencePassthrough,
}
