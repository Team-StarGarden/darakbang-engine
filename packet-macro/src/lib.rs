use darling::{Error, FromMeta, FromVariant};
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse_macro_input, parse_str, spanned::Spanned, Attribute, AttributeArgs, Fields, ItemEnum,
    Type,
};

#[derive(Debug, FromMeta)]
struct PacketItem {
    namespace: String,
    handler_target: String,
}

// uses_type_params!(PacketItem, handler_target);

#[derive(Debug, FromVariant)]
#[darling(attributes(packet))]
struct PacketVariant {
    id: String,
    attrs: Vec<Attribute>,
}

#[proc_macro_attribute]
pub fn packet(args: TokenStream, item: TokenStream) -> TokenStream {
    let attr_args = parse_macro_input!(args as AttributeArgs);
    let attr_span = attr_args.first().and_then(|first| {
        first.span().join(
            attr_args
                .last()
                .expect("If first exists, last exists")
                .span(),
        )
    });

    let attr_args = match PacketItem::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => {
            return e.write_errors().into();
        }
    };

    let item = parse_macro_input!(item as ItemEnum);

    let mut variant_generated_client = Vec::new();
    let mut variant_generated_server = Vec::new();
    let mut variant_generated_names = Vec::new();

    for variant in &item.variants {
        let variant_attrs = match PacketVariant::from_variant(&variant) {
            Ok(v) => v,
            Err(e) => {
                return e.write_errors().into();
            }
        };
        let attrs: Vec<_> = variant_attrs.attrs;
        let name = &variant.ident;
        let rename = format!("{}:{}", attr_args.namespace, variant_attrs.id);
        match &variant.fields {
            Fields::Named(named) => {
                let req_type = named.named.iter().find_map(|field| match &field.ident {
                    Some(v) if v.to_string() == "request" => Some(field.ty.clone()),
                    _ => None,
                });
                let req_type = if let Some(req_type) = req_type {
                    req_type
                } else {
                    return Error::missing_field("request")
                        .with_span(named)
                        .write_errors()
                        .into();
                };
                let res_type = named.named.iter().find_map(|field| match &field.ident {
                    Some(v) if v.to_string() == "response" => Some(field.ty.clone()),
                    _ => None,
                });
                let res_type = if let Some(res_type) = res_type {
                    res_type
                } else {
                    return Error::missing_field("response")
                        .with_span(named)
                        .write_errors()
                        .into();
                };
                variant_generated_server.push(quote! {
                    #(#attrs)*
                    #[serde(rename = #rename)]
                    #name {
                        ok: bool,
                        body: #res_type,
                    }
                });
                variant_generated_client.push(quote! {
                    #(#attrs)*
                    #[serde(rename = #rename)]
                    #name {
                        body: #req_type,
                    }
                });
                variant_generated_names.push(name.clone());
            }
            Fields::Unnamed(unnamed) => variant_generated_server.push(quote! {
                #(#attrs)*
                #[serde(rename = #rename)]
                #name {
                    ok: bool,
                    body: #unnamed,
                }
            }),
            Fields::Unit => {
                return Error::unsupported_shape(&name.to_string())
                    .with_span(&variant)
                    .write_errors()
                    .into();
            }
        }
    }

    let vis = item.vis;
    let name = item.ident;
    let server_name = format_ident!("{}Server", name);
    let client_name = format_ident!("{}Client", name);
    let generics = item.generics;
    let attrs = item.attrs;
    let handler_target: Type = if let Ok(handler_target) = parse_str(&attr_args.handler_target) {
        handler_target
    } else {
        let mut error = Error::custom(format_args!(
            "Expected type in string literal, but {} received",
            attr_args.handler_target
        ));
        if let Some(attr_span) = attr_span {
            error = error.with_span(&attr_span);
        }
        return error.write_errors().into();
    };

    (quote! {
        #[derive(serde::Serialize, actix::MessageResponse)]
        #[serde(tag = "kind")]
        #(#attrs)*
        #vis enum #server_name #generics {
            #(#variant_generated_server,)*
        }
        #[derive(serde::Deserialize)]
        #[serde(tag = "kind")]
        #(#attrs)*
        #vis enum #client_name #generics {
            #(#variant_generated_client,)*
        }
        impl actix::Message for #client_name {
            type Result = #server_name;
        }

        impl actix::Handler<#client_name> for #handler_target {
            type Result = #server_name;
            fn handle(&mut self, message: #client_name, ctx: &mut Self::Context) -> Self::Result {
                match message {
                    #(
                        #client_name::#variant_generated_names { body } => {
                            let result = self.handle(body, ctx);
                            #server_name::#variant_generated_names {
                                ok: result.is_ok(),
                                body: result,
                            }
                        }
                    ),*
                }
            }
        }
    })
    .into()
}
