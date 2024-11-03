pub struct Grammar(Vec<Production>);

pub struct Production(Name, Or2<Choice, Link>);

pub struct NameStartChar();
