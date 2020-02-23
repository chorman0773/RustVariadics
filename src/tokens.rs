use syn::export::TokenStream;
use syn::parse::Parse;
use syn::token::Token;
use syn::buffer::Cursor;

/// Keyword inside of expanding blocks to deliminate a fold-expression
/// A fold expression has form fold($pattern $op ...) (left-associative) or fold (... $op $pattern) (right-associative),
///  where $pattern is an expandable pattern that contains at least one unexpanded parameter pack,
///  and $op is some binary operator.
///
/// Fold expressions will not work with empty parameter packs. Any pack that will be used with fold must be declared nonempty at some level.
syn::custom_keyword!(fold);
/// Takes the last element of a parameter pack or indicates that a bound (usually ?Sized) applies only to the last element of a parameter pack
/// Syntax: last($pack) where pack is an unexpanded parameter pack.
/// last $bound where $bound is a generic bound.
syn::custom_keyword!(last);

/// Used a declaration of a parameter pack to indicate that the pack only matches nonempty expansions.
/// This is usually used in a generic pack declaration, but can appear at any level.
/// nonempty T... is the same as size(1..) T...
syn::custom_keyword!(nonempty);

/// Special Pattern that expands to the number of elements in a named pack. Use:
/// sizeof...($pack) where $pack is an unexpanded parameter pack.
/// This expands to a literal with type u8.
syn::custom_punctuation!(SizeOfPack,sizeof...);

/// Indicates that a parameter pack has a particular or range of valid sizes.
/// When used with a specific value, the value is usually obtained from sizeof...
syn::custom_keyword!(size);