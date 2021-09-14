use proc_macro::{Group, Ident, TokenStream, TokenTree};

fn replace_ident(ident: Ident) -> Option<TokenTree> {
    let ident_str = ident.to_string();

    let new_str = match ident_str.as_str() {
        "中" => "std",
        "仓" => "collections",
        "错" => "Err",
        "好" => "Ok",
        "串" => "String",
        "典" => "HashMap",
        "标" => "Default",
        "误" => "Error",
        "或" => "Option",
        "有" => "Some",
        "无" => "None",
        "果" => "Result",
        "自" => "Self",
        "印" => "println",
        "断" => "break",
        "另" => "async",
        "等" => "await",
        "环" => "loop",
        "移" => "move",
        "箱" => "crate",
        "禁" => "unreachable_code",
        "作" => "as",
        "常" => "const",
        "性" => "trait",
        "危" => "unsafe",
        "在" => "in",
        "从" => "from",
        "动" => "dyn",
        "解" => "unwrap",
        "准" => "default",
        "作引" => "as_ref",
        "言" => "io",
        "外" => "extern",
        "假" => "false",
        "函" => "fn",
        "超" => "super",
        "入" => "insert",
        "取" => "get",
        "匀" => "allow",
        "警" => "panic",
        "模" => "mod",
        "变" => "mut",
        "新" => "new",
        "于" => "where",
        "令" | "为" => "for",
        "取入" => "get_or_insert_with",
        "主" => "main",
        "公" => "pub",
        "无或" => None?,
        "返" => "return",
        "阐" => "impl",
        "引" => "ref",
        "配" => "match",
        "若" => "if",
        "否则" => "else",
        "身" => "self",
        "定" => "let",
        "静" => "static",
        "构" => "struct",
        "期" => "expect",
        "当" => "while",
        "用" => "use",
        "进" => "into",
        "真" => "true",
        "枚" => "enum",
        "锈" => "xiu",

        _ => &ident_str,
    };

    let new_ident = Ident::new(new_str, ident.span());
    Some(TokenTree::Ident(new_ident))
}

fn replace_tree(tok: TokenTree, out: &mut Vec<TokenTree>) {
    match tok {
        TokenTree::Group(group) => {
            let mut group_elem = Vec::new();
            replace_stream(group.stream(), &mut group_elem);
            let mut new_stream = TokenStream::new();
            new_stream.extend(group_elem);
            out.push(TokenTree::Group(Group::new(group.delimiter(), new_stream)));
        }
        TokenTree::Ident(ident) => {
            if let Some(ident) = replace_ident(ident) {
                out.push(ident);
            }
        }
        TokenTree::Punct(..) | TokenTree::Literal(..) => {
            out.push(tok);
        }
    }
}

fn replace_stream(ts: TokenStream, out: &mut Vec<TokenTree>) {
    for tok in ts {
        replace_tree(tok, out)
    }
}

#[proc_macro]
pub fn 锈(item: TokenStream) -> TokenStream {
    let mut returned = Vec::new();
    replace_stream(item, &mut returned);
    let mut out = TokenStream::new();
    out.extend(returned);
    out
}
