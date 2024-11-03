#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

pub struct document(prolog, element, Vec<Misc>, EOF);

pub struct prolog(Option<XMLDecl>, Vec<Misc>, Option<(doctypedelc, Vec<Misc>)>);

pub struct XMLDecl(VersionInfo, Option<EncodingDecl>, Option<SDDecl>, Option<S>);

pub struct VersionInfo(S, Eq, OneOf2<VersionNum, VersionNum>);

pub struct Eq(Option<S>, Option<S>);

pub struct Misc(OneOf3<Comment, PI, S>);

pub struct doctypedecl(
    S,
    Name,
    Option<(S, ExternalID)>,
    Option<S>,
    Option<(intSubset, Option<S>)>,
);

pub struct DeclSep(OneOf2<PEReference, S>);
