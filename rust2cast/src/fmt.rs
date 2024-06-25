use crate::{
    stmt::CStatement, CDeclaration, CEnum, CExpr, CFunction, CProgram, CStruct, CType, CUnion,
};

impl CType {
    fn to_c_code(&self) -> String {
        match self {
            CType::Int => "int".to_string(),
            CType::Float => "float".to_string(),
            CType::Double => "double".to_string(),
            CType::Char => "char".to_string(),
            CType::Void => "void".to_string(),
            CType::Struct(name) => format!("struct {}", name),
            CType::Union(name) => format!("union {}", name),
            CType::Enum(name) => format!("enum {}", name),
            CType::Pointer(ctype) => format!("{}*", ctype.to_c_code()),
            CType::Array(ctype, size) => format!("{}[{}]", ctype.to_c_code(), size),
            CType::Function(ret_type, params) => format!(
                "{} (*)({})",
                ret_type.to_c_code(),
                params
                    .iter()
                    .map(|p| p.to_c_code())
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
        }
    }
}

impl CExpr {
    fn to_c_code(&self) -> String {
        match self {
            CExpr::Literal(value) => value.clone(),
            CExpr::Variable(name) => name.clone(),
            CExpr::BinaryOp(op, expr1, expr2) => {
                format!("({} {} {})", expr1.to_c_code(), op, expr2.to_c_code())
            }
            CExpr::UnaryOp(op, expr) => format!("({}{})", op, expr.to_c_code()),
            CExpr::FunctionCall(name, args) => format!(
                "{}({})",
                name,
                args.iter()
                    .map(|arg| arg.to_c_code())
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            CExpr::StructAccess(expr, field) => format!("{}.{}", expr.to_c_code(), field),
            CExpr::ArrayAccess(expr, index) => {
                format!("{}[{}]", expr.to_c_code(), index.to_c_code())
            }
            CExpr::Cast(ctype, expr) => format!("({}){}", ctype.to_c_code(), expr.to_c_code()),
            CExpr::Sizeof(ctype) => format!("sizeof({})", ctype.to_c_code()),
        }
    }
}

impl CDeclaration {
    fn to_c_code(&self) -> String {
        let initializer_code = match &self.initializer {
            Some(expr) => format!(" = {}", expr.to_c_code()),
            None => "".to_string(),
        };
        format!(
            "{} {}{}",
            self.ctype.to_c_code(),
            self.name,
            initializer_code
        )
    }
}

impl CStruct {
    fn to_c_code(&self) -> String {
        let fields_code = self
            .fields
            .iter()
            .map(|field| field.to_c_code())
            .collect::<Vec<String>>()
            .join(";\n");
        format!("struct {} {{\n{}\n}}", self.name, fields_code)
    }
}

impl CUnion {
    fn to_c_code(&self) -> String {
        let fields_code = self
            .fields
            .iter()
            .map(|field| field.to_c_code())
            .collect::<Vec<String>>()
            .join(";\n");
        format!("union {} {{\n{}\n}}", self.name, fields_code)
    }
}

impl CEnum {
    fn to_c_code(&self) -> String {
        let variants_code = self
            .variants
            .iter()
            .map(|(name, value)| match value {
                Some(value) => format!("{} = {}", name, value),
                None => name.clone(),
            })
            .collect::<Vec<String>>()
            .join(",\n");
        format!("enum {} {{\n{}\n}}", self.name, variants_code)
    }
}

impl CStatement {
    fn to_c_code(&self) -> String {
        match self {
            CStatement::Declaration(decl) => format!("{};", decl.to_c_code()),
            CStatement::Assignment(expr1, expr2) => {
                format!("{} = {};", expr1.to_c_code(), expr2.to_c_code())
            }
            CStatement::Return(expr) => format!("return {};", expr.to_c_code()),
            CStatement::If(cond, then_block, else_block) => format!(
                "if ({}) {{\n{}\n}} else {{\n{}\n}}",
                cond.to_c_code(),
                block_to_c_code(then_block),
                block_to_c_code(else_block)
            ),
            CStatement::While(cond, block) => format!(
                "while ({}) {{\n{}\n}}",
                cond.to_c_code(),
                block_to_c_code(block)
            ),
            CStatement::DoWhile(cond, block) => format!(
                "do {{\n{}\n}} while ({});",
                block_to_c_code(block),
                cond.to_c_code()
            ),
            CStatement::For(init, cond, update, block) => format!(
                "for ({}; {}; {}) {{\n{}\n}}",
                init.to_c_code(),
                cond.to_c_code(),
                update.to_c_code(),
                block_to_c_code(block)
            ),
            CStatement::Switch(expr, cases, default_case) => {
                let cases_code = cases
                    .iter()
                    .map(|(case_expr, case_block)| {
                        format!(
                            "case {}: {{\n{}\n}}",
                            case_expr.to_c_code(),
                            block_to_c_code(case_block)
                        )
                    })
                    .collect::<Vec<String>>()
                    .join("\n");
                let default_case_code = match default_case {
                    Some(block) => format!("default: {{\n{}\n}}", block_to_c_code(block)),
                    None => "".to_string(),
                };
                format!(
                    "switch ({}) {{\n{}\n{}\n}}",
                    expr.to_c_code(),
                    cases_code,
                    default_case_code
                )
            }
            CStatement::Break => "break;".to_string(),
            CStatement::Continue => "continue;".to_string(),
            CStatement::Goto(label) => format!("goto {};", label),
            CStatement::Label(label, stmt) => format!("{}: {}", label, stmt.to_c_code()),
            CStatement::Block(block) => format!("{{\n{}\n}}", block_to_c_code(block)),
            CStatement::Expression(expr) => format!("{};", expr.to_c_code()),
        }
    }
}

impl CFunction {
    fn to_c_code(&self) -> String {
        let params_code = self
            .parameters
            .iter()
            .map(|param| param.to_c_code())
            .collect::<Vec<String>>()
            .join(", ");
        let body_code = block_to_c_code(&self.body);
        format!(
            "{} {}({}) {{\n{}\n}}",
            self.return_type.to_c_code(),
            self.name,
            params_code,
            body_code
        )
    }
}

impl CProgram {
    fn to_c_code(&self) -> String {
        let structs_code = self
            .structs
            .iter()
            .map(|s| s.to_c_code())
            .collect::<Vec<String>>()
            .join("\n");
        let unions_code = self
            .unions
            .iter()
            .map(|u| u.to_c_code())
            .collect::<Vec<String>>()
            .join("\n");
        let enums_code = self
            .enums
            .iter()
            .map(|e| e.to_c_code())
            .collect::<Vec<String>>()
            .join("\n");
        let functions_code = self
            .functions
            .iter()
            .map(|f| f.to_c_code())
            .collect::<Vec<String>>()
            .join("\n");
        let global_vars_code = self
            .global_variables
            .iter()
            .map(|v| v.to_c_code())
            .collect::<Vec<String>>()
            .join(";\n");
        format!(
            "{}\n{}\n{}\n{}\n{}",
            structs_code, unions_code, enums_code, functions_code, global_vars_code
        )
    }
}

fn block_to_c_code(block: &Vec<CStatement>) -> String {
    block
        .iter()
        .map(|stmt| stmt.to_c_code())
        .collect::<Vec<String>>()
        .join("\n")
}
