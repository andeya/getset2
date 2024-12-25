use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use proc_macro_error2::abort;
use syn::{self, ext::IdentExt, spanned::Spanned, Field, Visibility};

use crate::parse_attr;

use self::GenMode::{GetCopy, GetMut, GetRef, Set, SetWith};

#[derive(Clone)]
pub struct GenParams {
    pub mode: GenMode,
    pub vis: Option<Visibility>,
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub enum GenMode {
    GetRef,
    GetCopy,
    GetMut,
    Set,
    SetWith,
}

impl GenMode {
    pub fn list() -> [GenMode; 5] {
        [
            GenMode::GetRef,
            GenMode::GetCopy,
            GenMode::GetMut,
            GenMode::Set,
            GenMode::SetWith,
        ]
    }

    pub fn prefix(self) -> &'static str {
        match self {
            GetRef | GetCopy | GetMut => "",
            Set => "set_",
            SetWith => "with_",
        }
    }

    pub fn suffix(self) -> &'static str {
        match self {
            GetRef | GetCopy | Set | SetWith => "",
            GetMut => "_mut",
        }
    }
}

pub fn implement(field: &Field, global_params: &[GenParams]) -> TokenStream2 {
    let mut ts = TokenStream2::new();
    let Some(attr) = field
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("getset2"))
    else {
        return ts;
    };
    let (mut params_list, skip_list) = parse_attr(attr);
    let had_ref_copy = params_list
        .iter()
        .any(|p| matches!(p.mode, GenMode::GetRef | GenMode::GetCopy));
    for params in global_params {
        if (!skip_list.contains(&params.mode) && params_list.iter().all(|p| p.mode != params.mode))
            && (!had_ref_copy || !matches!(params.mode, GenMode::GetRef | GenMode::GetCopy))
        {
            params_list.push(params.clone());
        }
    }
    for mut params in params_list {
        params.vis = params.vis.or_else(|| Some(field.vis.clone()));
        ts.extend(gen_method(field, params));
    }
    ts
}

pub fn gen_method(field: &Field, params: GenParams) -> TokenStream2 {
    let field_name = field
        .ident
        .clone()
        .unwrap_or_else(|| abort!(field.span(), "Expected the field to have a name"));

    let fn_name = if params.mode.prefix().is_empty() && params.mode.suffix().is_empty() {
        field_name.clone()
    } else {
        Ident::new(
            &format!(
                "{}{}{}",
                params.mode.prefix(),
                field_name.unraw(),
                params.mode.suffix()
            ),
            Span::call_site(),
        )
    };
    let ty = field.ty.clone();

    let doc = field.attrs.iter().filter(|v| v.meta.path().is_ident("doc"));

    let visibility = params.vis;

    match params.mode {
        GenMode::GetRef => {
            quote! {
                #(#doc)*
                #[inline(always)]
                #visibility fn #fn_name(&self) -> &#ty {
                    &self.#field_name
                }
            }
        }
        GenMode::GetCopy => {
            quote! {
                #(#doc)*
                #[inline(always)]
                #visibility fn #fn_name(&self) -> #ty {
                    self.#field_name
                }
            }
        }
        GenMode::Set => {
            quote! {
                #(#doc)*
                #[inline(always)]
                #visibility fn #fn_name(&mut self, val: #ty) -> &mut Self {
                    self.#field_name = val;
                    self
                }
            }
        }
        GenMode::GetMut => {
            quote! {
                #(#doc)*
                #[inline(always)]
                #visibility fn #fn_name(&mut self) -> &mut #ty {
                    &mut self.#field_name
                }
            }
        }
        GenMode::SetWith => {
            quote! {
                #(#doc)*
                #[inline(always)]
                #visibility fn #fn_name(mut self, val: #ty) -> Self {
                    self.#field_name = val;
                    self
                }
            }
        }
    }
}
