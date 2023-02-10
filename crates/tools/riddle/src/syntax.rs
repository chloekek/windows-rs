use metadata::writer;
use syn::{parse::*, spanned::*, *};
use std::collections::*;

mod keywords {
    syn::custom_keyword!(interface);
    syn::custom_keyword!(class);
}

pub struct File {
    references: Vec<ItemUse>,
    modules: Vec<Module>,
}

impl Parse for File {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut references = vec![];
        let mut modules = vec![];
        while !input.is_empty() {
            let lookahead = input.lookahead1();
            if lookahead.peek(Token![mod]) {
                modules.push(input.parse()?);
            } else if lookahead.peek(Token![use]) {
                references.push(input.parse()?);
            } else {
                return Err(lookahead.error());
            }
        }
        Ok(Self { references, modules })
    }
}

impl File {
    pub fn use_names(&self) ->Result<HashMap<String, String>> {
        fn walk(tree: &UseTree, namespace: &mut String, references: &mut HashMap<String, String>) -> Result<()> {
            match tree {
                UseTree::Path(input) => {
                    if !namespace.is_empty() { namespace.push('.') };
                    namespace.push_str(&input.ident.to_string());
                    walk(&input.tree, namespace, references)?;
                }
                UseTree::Name(input) => {
                    match references.entry(input.ident.to_string()) {
                        hash_map::Entry::Vacant(entry) => {
                            // TODO: check wether this type exists in the references assemblies
                            entry.insert(namespace.to_string());
                        }
                        _ => return Err(Error::new(input.span(), "ambigious name")),
                    }

                }
                UseTree::Group(input) => {
                    for tree in &input.items {
                        walk(tree, namespace, references);
                    }
                }
                UseTree::Rename(_) => return Err(Error::new(tree.span(), "rename not supported")),
                UseTree::Glob(_) => return Err(Error::new(tree.span(), "glob not supported")),
            }
            Ok(())
        }

        let mut references = HashMap::new();

        for item in &self.references {
            walk(&item.tree,&mut String::new(), &mut references);
        }

        Ok(references)
    }

    pub fn to_writer(&self, items: &mut Vec<writer::Item>) -> Result<()> {
        for module in &self.modules {
            module.to_writer(module.name.to_string(), items)?;
        }
        Ok(())
    }
}

struct Module {
    name: Ident,
    members: Vec<ModuleMember>,
}

impl Parse for Module {
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse::<Token![mod]>()?;
        let name = input.parse::<Ident>()?;
        let content;
        braced!(content in input);
        let mut members = vec![];
        while !content.is_empty() {
            members.push(content.parse::<ModuleMember>()?);
        }
        Ok(Self { name, members })
    }
}

impl Module {
    fn to_writer(&self, namespace: String, items: &mut Vec<writer::Item>) -> Result<()> {
        for member in &self.members {
            match member {
                ModuleMember::Module(member) => member.to_writer(format!("{namespace}.{}", member.name), items)?,
                ModuleMember::Interface(member) => member.to_writer(namespace.clone(), items)?,
                ModuleMember::Struct(member) => member.to_writer(namespace.clone(), items)?,
                ModuleMember::Enum(member) => member.to_writer(namespace.clone(), items)?,
                ModuleMember::Class(member) => member.to_writer(namespace.clone(), items)?,
            }
        }
        Ok(())
    }
}

enum ModuleMember {
    Module(Module),
    Interface(Interface),
    Struct(Struct),
    Enum(Enum),
    Class(Class),
}

impl Parse for ModuleMember {
    fn parse(input: ParseStream) -> Result<Self> {
        let attributes: Vec<Attribute> = input.call(Attribute::parse_outer)?;
        let lookahead = input.lookahead1();
        if lookahead.peek(Token![mod]) {
            if let Some(attribute) = attributes.first() {
                return Err(Error::new(attribute.span(), "module attribute are not supported"));
            }
            Ok(ModuleMember::Module(input.parse()?))
        } else if lookahead.peek(keywords::interface) {
            Ok(ModuleMember::Interface(Interface::parse(attributes, input)?))
        } else if lookahead.peek(Token![struct]) {
            Ok(ModuleMember::Struct(Struct::parse(attributes, input)?))
        } else if lookahead.peek(Token![enum]) {
            Ok(ModuleMember::Enum(Enum::parse(attributes, input)?))
        } else if lookahead.peek(keywords::class) {
            Ok(ModuleMember::Class(Class::parse(attributes, input)?))
        } else {
            Err(lookahead.error())
        }
    }
}

struct Class {
    attributes: Vec<Attribute>,
    name: Ident,
    extends: Vec<Path>,
}

impl Class {
    fn parse(attributes: Vec<Attribute>, input: ParseStream) -> Result<Self> {
        input.parse::<keywords::class>()?;
        let name = input.parse::<Ident>()?;
        let mut extends = Vec::new();

        if input.peek(Token![:]) {
            input.parse::<Token![:]>()?;
            while input.peek(Ident) {
                extends.push(input.parse::<Path>()?);
                let _ = input.parse::<Token![,]>();
            }
        }

        input.parse::<Token![;]>()?;
        Ok(Self { attributes, name, extends })
    }
}

impl Class {
    fn to_writer(&self, namespace: String, items: &mut Vec<writer::Item>) -> Result<()> {
        items.push(writer::Item::Class(writer::Class { namespace, name: self.name.to_string(), attributes: attributes_to_attributes(&self.attributes)? }));
        Ok(())
    }
}

struct Interface {
    attributes: Vec<Attribute>,
    name: Ident,
    methods: Vec<TraitItemMethod>,
}

impl Interface {
    fn parse(attributes: Vec<Attribute>, input: ParseStream) -> Result<Self> {
        input.parse::<keywords::interface>()?;
        let name = input.parse::<Ident>()?;
        let content;
        braced!(content in input);
        let mut methods = vec![];
        while !content.is_empty() {
            methods.push(content.parse::<TraitItemMethod>()?);
        }
        Ok(Self { attributes, name, methods })
    }
}

impl Interface {
    fn to_writer(&self, namespace: String, items: &mut Vec<writer::Item>) -> Result<()> {
        let mut methods = vec![];

        for method in &self.methods {
            let return_type = match &method.sig.output {
                ReturnType::Default => writer::Type::Void,
                ReturnType::Type(_, ty) => type_to_type(&namespace, ty)?,
            };
            let mut params = vec![];
            for arg in &method.sig.inputs {
                let FnArg::Typed(pat_type) = arg else {
                    continue;
                };

                let mut flags = writer::ParamFlags::INPUT;

                for attribute in &pat_type.attrs {
                    match path_to_string(&attribute.path).as_str() {
                        "out" => flags = writer::ParamFlags::OUTPUT,
                        _ => return Err(Error::new(attribute.path.span(), "unsupported attribute")),
                    }
                }

                let Pat::Ident(pat_ident) = &*pat_type.pat else {
                    return Err(Error::new(pat_type.pat.span(), "expected parameter name"));
                };

                params.push(writer::Param { name: pat_ident.ident.to_string(), ty: type_to_type(&namespace, &pat_type.ty)?, flags });
            }
            methods.push(writer::Method { name: method.sig.ident.to_string(), return_type, params });
        }

        items.push(writer::Item::Interface(writer::Interface { namespace, name: self.name.to_string(), methods }));
        Ok(())
    }
}

struct Struct {
    item: ItemStruct,
}

impl Struct {
    fn parse(attributes: Vec<Attribute>, input: ParseStream) -> Result<Self> {
        let mut item: ItemStruct = input.parse()?;
        item.attrs = attributes;
        Ok(Self { item })
    }

    fn to_writer(&self, namespace: String, items: &mut Vec<writer::Item>) -> Result<()> {
        let mut fields = vec![];

        let Fields::Named(named) = &self.item.fields else {
            return Err(Error::new(self.item.fields.span(), "expected named fields"));
        };

        for field in &named.named {
            fields.push(writer::Field { name: field.ident.as_ref().unwrap().to_string(), ty: type_to_type(&namespace, &field.ty)? });
        }

        items.push(writer::Item::Struct(writer::Struct { namespace, name: self.item.ident.to_string(), fields }));
        Ok(())
    }
}

struct Enum {
    item: ItemEnum,
}

impl Enum {
    fn parse(attributes: Vec<Attribute>, input: ParseStream) -> Result<Self> {
        let mut item: ItemEnum = input.parse()?;
        item.attrs = attributes;
        Ok(Enum { item })
    }

    fn to_writer(&self, namespace: String, items: &mut Vec<writer::Item>) -> Result<()> {
        let mut constants = vec![];
        let mut value = 0;

        // TODO: need to read the `#[repr(u8)]` attribute infer the underlying type

        for variant in &self.item.variants {
            if let Some((_, discriminant)) = &variant.discriminant {
                let Expr::Lit(discriminant) = discriminant else {
                    return Err(Error::new(discriminant.span(), "expected literal discriminant"));
                };

                let Lit::Int(discriminant) = &discriminant.lit else {
                    return Err(Error::new(discriminant.lit.span(), "expected integer discriminant"));
                };

                value = discriminant.base10_parse()?;
            }

            constants.push(writer::Constant { name: variant.ident.to_string(), value: writer::Value::I32(value) });
            value += 1;
        }

        items.push(writer::Item::Enum(writer::Enum { namespace, name: self.item.ident.to_string(), constants }));
        Ok(())
    }
}

fn type_to_type(namespace: &str, ty: &Type) -> Result<writer::Type> {
    let Type::Path(path) = ty else {
        return Err(Error::new(ty.span(), "expected type path"));
    };

    let name = path_to_string(&path.path);

    let ty = match name.as_str() {
        "bool" => writer::Type::Bool,
        "i8" => writer::Type::I8,
        "u8" => writer::Type::U8,
        "i16" => writer::Type::I16,
        "u16" => writer::Type::U16,
        "i32" => writer::Type::I32,
        "u32" => writer::Type::U32,
        "i64" => writer::Type::I64,
        "u64" => writer::Type::U64,
        "f32" => writer::Type::F32,
        "f64" => writer::Type::F64,
        "isize" => writer::Type::ISize,
        "usize" => writer::Type::USize,
        _ => {
            if let Some((namespace, name)) = name.rsplit_once('.') {
                writer::Type::Named((namespace.to_string(), name.to_string()))
            } else {
                writer::Type::Named((namespace.to_string(), name.to_string()))
            }
        }
    };

    Ok(ty)
}

fn path_to_string(path: &Path) -> String {
    let mut name = String::new();

    for segment in &path.segments {
        if !name.is_empty() {
            name.push('.');
        }
        name.push_str(&segment.ident.to_string());
    }

    name
}

fn attribute_to_attribute(attribute:&Attribute) -> Result<writer::Attribute> {
    let attribute = attribute.parse_meta()?;
    let path = match &attribute {
        Meta::Path(path) => &path,
        Meta::List(list) => &list.path, // TODO: grab values
        Meta::NameValue(_) => return Err(Error::new(attribute.span(), "attribute list expected")),
    };
    let path = path_to_string(&path);
    let Some((namespace, name)) = path.rsplit_once('.') else {
        return Err(Error::new(attribute.span(), "qualified attribute expected"));
    };
    Ok(writer::Attribute{ namespace: namespace.to_string(), name: name.to_string(), args: vec![] })    
}

fn attributes_to_attributes(attributes:&[Attribute]) -> Result<Vec<writer::Attribute>> {
    let mut result = vec![];
    for attribute in attributes.iter() {
        result.push(attribute_to_attribute(attribute)?);
    }
    Ok(result)
}
