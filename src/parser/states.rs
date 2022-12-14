use super::super::lex::Token;

#[derive(Debug, Clone)]
pub enum WState {
    Prog,
    Stmt,
    IfStmt,
    IfExt,
    ElseExt,
    WhileStmt,
    AssignStmt,
    UpdateStmt,
    Expr,
    Expre,
    Term,
    Termt,
    CntPt,
    CntPtc,
    Factor,
    Args,
    ArgsA,
    NoSemiUpdateStmt,
    Terminal(Token),
    Empty,
    ForStmt,
    FunctionDecl,
}
impl WState {
    pub fn is_terminal(&self) -> bool {
        let mut res = false;
        if let WState::Terminal(_) = self {
            res = true
        }
        if let WState::Empty = self {
            res = true
        }
        res
    }
}
