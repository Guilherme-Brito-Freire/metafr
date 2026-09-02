///
/// This file is auto generated to map the CSS tags!
/// 
/// This file must to be hardCoded to improve the perfomace
/// 

use crate::param::{create_param, Param};

pub enum StyleTag {
    // Layout & Display
    Display,
    Position,
    Top,
    Right,
    Bottom,
    Left,
    ZIndex,
    Float,
    Clear,
    Overflow,
    OverflowX,
    OverflowY,
    Visibility,

    // Box Model & Sizing
    Width,
    Height,
    MinWidth,
    MinHeight,
    MaxWidth,
    MaxHeight,
    Margin,
    MarginTop,
    MarginRight,
    MarginBottom,
    MarginLeft,
    Padding,
    PaddingTop,
    PaddingRight,
    PaddingBottom,
    PaddingLeft,
    BoxSizing,
    AspectRatio,

    // Flexbox
    Flex,
    FlexDirection,
    FlexWrap,
    FlexFlow,
    JustifyContent,
    AlignItems,
    AlignContent,
    AlignSelf,
    FlexGrow,
    FlexShrink,
    FlexBasis,
    Order,
    Gap,
    RowGap,
    ColumnGap,

    // Grid Layout
    Grid,
    GridTemplateColumns,
    GridTemplateRows,
    GridTemplateAreas,
    GridAutoColumns,
    GridAutoRows,
    GridAutoFlow,
    GridColumn,
    GridRow,
    GridArea,

    // Colors & Backgrounds
    Color,
    Opacity,
    Background,
    BackgroundColor,
    BackgroundImage,
    BackgroundRepeat,
    BackgroundPosition,
    BackgroundSize,
    BackgroundAttachment,
    BackgroundClip,
    BackgroundOrigin,

    // Typography & Text
    Font,
    FontFamily,
    FontSize,
    FontWeight,
    FontStyle,
    FontVariant,
    LineHeight,
    LetterSpacing,
    WordSpacing,
    TextAlign,
    TextDecoration,
    TextDecorationLine,
    TextDecorationColor,
    TextDecorationStyle,
    TextTransform,
    TextIndent,
    TextOverflow,
    TextShadow,
    WhiteSpace,
    WordBreak,
    OverflowWrap,
    VerticalAlign,

    // Borders & Outlines
    Border,
    BorderWidth,
    BorderStyle,
    BorderColor,
    BorderTop,
    BorderRight,
    BorderBottom,
    BorderLeft,
    BorderRadius,
    BorderTopLeftRadius,
    BorderTopRightRadius,
    BorderBottomLeftRadius,
    BorderBottomRightRadius,
    Outline,
    OutlineWidth,
    OutlineStyle,
    OutlineColor,
    OutlineOffset,
    BoxShadow,

    // Transforms & Transitions
    Transform,
    TransformOrigin,
    Transition,
    TransitionProperty,
    TransitionDuration,
    TransitionTimingFunction,
    TransitionDelay,

    // Animations
    Animation,
    AnimationName,
    AnimationDuration,
    AnimationTimingFunction,
    AnimationDelay,
    AnimationIterationCount,
    AnimationDirection,
    AnimationFillMode,
    AnimationPlayState,

    // Effects & Filters
    Filter,
    BackdropFilter,
    MixBlendMode,
    ClipPath,
    Mask,

    // Interactivity & Tables
    Cursor,
    PointerEvents,
    UserSelect,
    Resize,
    ListStyle,
    ListStyleType,
    ListStylePosition,
    ListStyleImage,
    TableLayout,
    BorderCollapse,
    BorderSpacing,

    // Fallback
    Custom(&'static str),
}

impl StyleTag {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Display => "Display",
            Self::Position => "Position",
            Self::Top => "Top",
            Self::Right => "Right",
            Self::Bottom => "Bottom",
            Self::Left => "Left",
            Self::ZIndex => "ZIndex",
            Self::Float => "Float",
            Self::Clear => "Clear",
            Self::Overflow => "Overflow",
            Self::OverflowX => "OverflowX",
            Self::OverflowY => "OverflowY",
            Self::Visibility => "Visibility",
            Self::Width => "Width",
            Self::Height => "Height",
            Self::MinWidth => "MinWidth",
            Self::MinHeight => "MinHeight",
            Self::MaxWidth => "MaxWidth",
            Self::MaxHeight => "MaxHeight",
            Self::Margin => "Margin",
            Self::MarginTop => "MarginTop",
            Self::MarginRight => "MarginRight",
            Self::MarginBottom => "MarginBottom",
            Self::MarginLeft => "MarginLeft",
            Self::Padding => "Padding",
            Self::PaddingTop => "PaddingTop",
            Self::PaddingRight => "PaddingRight",
            Self::PaddingBottom => "PaddingBottom",
            Self::PaddingLeft => "PaddingLeft",
            Self::BoxSizing => "BoxSizing",
            Self::AspectRatio => "AspectRatio",
            Self::Flex => "Flex",
            Self::FlexDirection => "FlexDirection",
            Self::FlexWrap => "FlexWrap",
            Self::FlexFlow => "FlexFlow",
            Self::JustifyContent => "JustifyContent",
            Self::AlignItems => "AlignItems",
            Self::AlignContent => "AlignContent",
            Self::AlignSelf => "AlignSelf",
            Self::FlexGrow => "FlexGrow",
            Self::FlexShrink => "FlexShrink",
            Self::FlexBasis => "FlexBasis",
            Self::Order => "Order",
            Self::Gap => "Gap",
            Self::RowGap => "RowGap",
            Self::ColumnGap => "ColumnGap",
            Self::Grid => "Grid",
            Self::GridTemplateColumns => "GridTemplateColumns",
            Self::GridTemplateRows => "GridTemplateRows",
            Self::GridTemplateAreas => "GridTemplateAreas",
            Self::GridAutoColumns => "GridAutoColumns",
            Self::GridAutoRows => "GridAutoRows",
            Self::GridAutoFlow => "GridAutoFlow",
            Self::GridColumn => "GridColumn",
            Self::GridRow => "GridRow",
            Self::GridArea => "GridArea",
            Self::Color => "Color",
            Self::Opacity => "Opacity",
            Self::Background => "Background",
            Self::BackgroundColor => "BackgroundColor",
            Self::BackgroundImage => "BackgroundImage",
            Self::BackgroundRepeat => "BackgroundRepeat",
            Self::BackgroundPosition => "BackgroundPosition",
            Self::BackgroundSize => "BackgroundSize",
            Self::BackgroundAttachment => "BackgroundAttachment",
            Self::BackgroundClip => "BackgroundClip",
            Self::BackgroundOrigin => "BackgroundOrigin",
            Self::Font => "Font",
            Self::FontFamily => "FontFamily",
            Self::FontSize => "FontSize",
            Self::FontWeight => "FontWeight",
            Self::FontStyle => "FontStyle",
            Self::FontVariant => "FontVariant",
            Self::LineHeight => "LineHeight",
            Self::LetterSpacing => "LetterSpacing",
            Self::WordSpacing => "WordSpacing",
            Self::TextAlign => "TextAlign",
            Self::TextDecoration => "TextDecoration",
            Self::TextDecorationLine => "TextDecorationLine",
            Self::TextDecorationColor => "TextDecorationColor",
            Self::TextDecorationStyle => "TextDecorationStyle",
            Self::TextTransform => "TextTransform",
            Self::TextIndent => "TextIndent",
            Self::TextOverflow => "TextOverflow",
            Self::TextShadow => "TextShadow",
            Self::WhiteSpace => "WhiteSpace",
            Self::WordBreak => "WordBreak",
            Self::OverflowWrap => "OverflowWrap",
            Self::VerticalAlign => "VerticalAlign",
            Self::Border => "Border",
            Self::BorderWidth => "BorderWidth",
            Self::BorderStyle => "BorderStyle",
            Self::BorderColor => "BorderColor",
            Self::BorderTop => "BorderTop",
            Self::BorderRight => "BorderRight",
            Self::BorderBottom => "BorderBottom",
            Self::BorderLeft => "BorderLeft",
            Self::BorderRadius => "BorderRadius",
            Self::BorderTopLeftRadius => "BorderTopLeftRadius",
            Self::BorderTopRightRadius => "BorderTopRightRadius",
            Self::BorderBottomLeftRadius => "BorderBottomLeftRadius",
            Self::BorderBottomRightRadius => "BorderBottomRightRadius",
            Self::Outline => "Outline",
            Self::OutlineWidth => "OutlineWidth",
            Self::OutlineStyle => "OutlineStyle",
            Self::OutlineColor => "OutlineColor",
            Self::OutlineOffset => "OutlineOffset",
            Self::BoxShadow => "BoxShadow",
            Self::Transform => "Transform",
            Self::TransformOrigin => "TransformOrigin",
            Self::Transition => "Transition",
            Self::TransitionProperty => "TransitionProperty",
            Self::TransitionDuration => "TransitionDuration",
            Self::TransitionTimingFunction => "TransitionTimingFunction",
            Self::TransitionDelay => "TransitionDelay",
            Self::Animation => "Animation",
            Self::AnimationName => "AnimationName",
            Self::AnimationDuration => "AnimationDuration",
            Self::AnimationTimingFunction => "AnimationTimingFunction",
            Self::AnimationDelay => "AnimationDelay",
            Self::AnimationIterationCount => "AnimationIterationCount",
            Self::AnimationDirection => "AnimationDirection",
            Self::AnimationFillMode => "AnimationFillMode",
            Self::AnimationPlayState => "AnimationPlayState",
            Self::Filter => "Filter",
            Self::BackdropFilter => "BackdropFilter",
            Self::MixBlendMode => "MixBlendMode",
            Self::ClipPath => "ClipPath",
            Self::Mask => "Mask",
            Self::Cursor => "Cursor",
            Self::PointerEvents => "PointerEvents",
            Self::UserSelect => "UserSelect",
            Self::Resize => "Resize",
            Self::ListStyle => "ListStyle",
            Self::ListStyleType => "ListStyleType",
            Self::ListStylePosition => "ListStylePosition",
            Self::ListStyleImage => "ListStyleImage",
            Self::TableLayout => "TableLayout",
            Self::BorderCollapse => "BorderCollapse",
            Self::BorderSpacing => "BorderSpacing",
            Self::Custom(name) => name,
        }
    }

    pub fn get_tag(&self, value: &str) -> String {

        // Treats the Name to HTML read
        let mut name_char_array: Vec<char> = self.as_str().chars().collect();
        name_char_array[0] = name_char_array[0].to_lowercase().nth(0).unwrap(); // this transform the fist character in a lowercase

        // Get intermediare characteres to add a -
        let first_character: char = name_char_array[0];
        name_char_array.remove(0);
        let mut final_char_array: Vec<char> = vec![];
        for item in name_char_array {
            if item.is_uppercase() {
                // applt the fix
                final_char_array.push('-');
                final_char_array.push(item.to_lowercase().nth(0).unwrap());
                continue;
            }
            final_char_array.push(item); // here the char is right
        }
        final_char_array.insert(0, first_character);

        // Return to String
        let result: String = final_char_array.into_iter().collect();

        return format!("{}: {};",result,value); // This format the CSS
    }
}

// This function return the entire param from many tags
pub fn get_style_inline(tags: Vec<String>) -> Param {
    let mut final_inline: String = "".to_string();

    for item in tags {
        final_inline.push_str(&item);
    }

    create_param("style", &final_inline)
}