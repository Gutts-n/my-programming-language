use std::io::{Result, Write};

pub fn define_ast<W: Write>(writer: &mut W, base_name: &str, types: &[(&str, &str)]) -> Result<()> {
    // Write the trait (base)
    writeln!(writer, "pub trait {} {{", base_name)?;
    writeln!(
        writer,
        "    fn accept<R>(&self, visitor: &mut dyn {}Visitor<R>) -> R;",
        base_name
    )?;
    writeln!(writer, "}}\n")?;

    // Write the visitor trait
    writeln!(writer, "pub trait {}Visitor<R> {{", base_name)?;
    for (class_name, _) in types {
        writeln!(
            writer,
            "    fn visit_{}_{}(&mut self, expr: &{}) -> R;",
            class_name.to_lowercase(),
            base_name.to_lowercase(),
            class_name
        )?;
    }
    writeln!(writer, "}}\n")?;

    // Write each struct and impl
    for (class_name, field_list) in types {
        define_type(writer, base_name, class_name, field_list)?;
    }

    Ok(())
}

fn define_type<W: Write>(
    writer: &mut W,
    base_name: &str,
    class_name: &str,
    field_list: &str,
) -> Result<()> {
    // Define struct
    writeln!(writer, "#[derive(Debug, Clone)]")?;
    writeln!(writer, "pub struct {} {{", class_name)?;

    let fields: Vec<&str> = field_list.split(", ").collect();
    for field in &fields {
        let parts: Vec<&str> = field.split_whitespace().collect();
        let (typ, name) = (parts[0], parts[1]);
        writeln!(writer, "    pub {}: {},", name, map_type(typ))?;
    }

    writeln!(writer, "}}\n")?;

    // Implement base trait
    writeln!(writer, "impl {} for {} {{", base_name, class_name)?;
    writeln!(
        writer,
        "    fn accept<R>(&self, visitor: &mut dyn {}Visitor<R>) -> R {{",
        base_name
    )?;
    writeln!(
        writer,
        "        visitor.visit_{}_{}(self)",
        class_name.to_lowercase(),
        base_name.to_lowercase()
    )?;
    writeln!(writer, "    }}")?;
    writeln!(writer, "}}\n")?;

    Ok(())
}

fn map_type(typ: &str) -> &str {
    match typ {
        "Expr" => "Box<Expr>",
        "Token" => "Token",
        "Object" => "Literal", // You can define `Literal` as a custom enum
        _ => typ,
    }
}
