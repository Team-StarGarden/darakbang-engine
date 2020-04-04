use darling::{Error, FromMeta, FromVariant};
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, AttributeArgs, Fields, ItemEnum};

#[derive(Debug, FromMeta)]
struct PacketItem {
    namespace: String,
}
#[derive(Debug, FromVariant)]
#[darling(attributes(packet))]
struct PacketVariant {
    id: String,
}

#[proc_macro_attribute]
pub fn packet(args: TokenStream, item: TokenStream) -> TokenStream {
    let attr_args = parse_macro_input!(args as AttributeArgs);

    let attr_args = match PacketItem::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(e.write_errors());
        }
    };

    let item = parse_macro_input!(item as ItemEnum);

    let mut variant_generated_client = Vec::new();
    let mut variant_generated_server = Vec::new();

    for variant in &item.variants {
        let variant_attrs = match PacketVariant::from_variant(&variant) {
            Ok(v) => v,
            Err(e) => {
                return e.write_errors().into();
            }
        };
        let attrs: Vec<_> = variant
            .attrs
            .iter()
            .filter(|it| match it.path.get_ident() {
                Some(v) if v.to_string() == "packet" => false,
                _ => true,
            })
            .collect();
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
    let attrs: Vec<_> = item
        .attrs
        .iter()
        .filter(|it| match it.path.get_ident() {
            Some(v) if v.to_string() == "packet" => false,
            _ => true,
        })
        .collect();

    (quote! {
        #[derive(serde::Serialize)]
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
    })
    .into()
}
