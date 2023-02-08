use metadata::writer;
use syn::{parse::*, spanned::*, *};

mod keywords {
    syn::custom_keyword!(interface);
    syn::custom_keyword!(class);
}

pub trait ToWriter {
    fn to_writer(&self, namespace: String, items: &mut Vec<writer::Item>) -> Result<()>;
}

pub struct File {
    pub references: Vec<ItemUse>,
    pub modules: Vec<Module>,
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
        Ok(Self{ references, modules })
    }
}

impl File {
    pub fn to_writer(&self, items: &mut Vec<writer::Item>) -> Result<()> {
        for module in &self.modules {
            module.to_writer(module.name.to_string(), items)?;
        }
        Ok(())
    }
}

pub struct Module {
    pub name: Ident,
    pub members: Vec<ModuleMember>,
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

impl ToWriter for Module {
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

pub enum ModuleMember {
    Module(Module),
    Interface(Interface),
    Struct(ItemStruct),
    Enum(ItemEnum),
    Class(Class),
}

impl Parse for ModuleMember {
    fn parse(input: ParseStream) -> Result<Self> {
        let attrs: Vec<Attribute> = input.call(Attribute::parse_outer)?;
        let lookahead = input.lookahead1();
        if lookahead.peek(Token![mod]) {
            if let Some(attr) = attrs.first() {
                return Err(Error::new(attr.span(), "module attribute are not supported"));
            }
            Ok(ModuleMember::Module(input.parse()?))
        } else if lookahead.peek(keywords::interface) {
            Ok(ModuleMember::Interface(Interface::parse(attrs, input)?))
        } else if lookahead.peek(Token![struct]) {
            let mut item: ItemStruct = input.parse()?;
            item.attrs = attrs;
            Ok(ModuleMember::Struct(item))
        } else if lookahead.peek(Token![enum]) {
            let mut item: ItemEnum = input.parse()?;
            item.attrs = attrs;
            Ok(ModuleMember::Enum(item))
        } else if lookahead.peek(keywords::class) {
            Ok(ModuleMember::Class(Class::parse(attrs, input)?))
        } else {
            Err(lookahead.error())
        }
    }
}

pub struct Class {
    pub attrs: Vec<Attribute>,
    pub name: Ident,
    pub extends: Vec<Path>,
}

impl Class {
    fn parse(attrs: Vec<Attribute>, input: ParseStream) -> Result<Self> {
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
        Ok(Self { attrs, name, extends })
    }
}

impl ToWriter for Class {
    fn to_writer(&self, namespace: String, items: &mut Vec<writer::Item>) -> Result<()> {
        items.push(writer::Item::Class(writer::Class { namespace, name: self.name.to_string() }));
        Ok(())
    }
}

pub struct Interface {
    pub attrs: Vec<Attribute>,
    pub name: Ident,
    pub methods: Vec<TraitItemMethod>,
}

impl Interface {
    fn parse(attrs: Vec<Attribute>, input: ParseStream) -> Result<Self> {
        input.parse::<keywords::interface>()?;
        let name = input.parse::<Ident>()?;
        let content;
        braced!(content in input);
        let mut methods = vec![];
        while !content.is_empty() {
            methods.push(content.parse::<TraitItemMethod>()?);
        }
        Ok(Self { attrs, name, methods })
    }
}

impl ToWriter for Interface {
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

impl ToWriter for ItemStruct {
    fn to_writer(&self, namespace: String, items: &mut Vec<writer::Item>) -> Result<()> {
        let mut fields = vec![];

        let Fields::Named(named) = &self.fields else {
            return Err(Error::new(self.fields.span(), "expected named fields"));
        };

        for field in &named.named {
            fields.push(writer::Field { name: field.ident.as_ref().unwrap().to_string(), ty: type_to_type(&namespace, &field.ty)? });
        }

        items.push(writer::Item::Struct(writer::Struct { namespace, name: self.ident.to_string(), fields }));
        Ok(())
    }
}

impl ToWriter for ItemEnum {
    fn to_writer(&self, namespace: String, items: &mut Vec<writer::Item>) -> Result<()> {
        let mut constants = vec![];
        let mut value = 0;

        // TODO: need to read the `#[repr(u8)]` attribute infer the underlying type

        for variant in &self.variants {
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

        items.push(writer::Item::Enum(writer::Enum { namespace, name: self.ident.to_string(), constants }));
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