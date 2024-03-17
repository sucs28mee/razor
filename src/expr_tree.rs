use std::ptr;

pub trait Operator {
    fn precedence(&self) -> u32;
}

#[derive(Debug, Clone)]
pub enum ExprTree<T, O> {
    Value(T),
    Expression {
        left: Box<ExprTree<T, O>>,
        operator: O,
        right: Box<ExprTree<T, O>>,
    },
    Enclosed(Box<ExprTree<T, O>>),
}

impl<T, O> ExprTree<T, O> {
    pub fn new_expression(left: ExprTree<T, O>, operator: O, right: ExprTree<T, O>) -> Self {
        ExprTree::Expression {
            operator,
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    pub fn new_enclosed(node: ExprTree<T, O>) -> Self {
        ExprTree::Enclosed(Box::new(node))
    }
}

impl<T, O: Operator> ExprTree<T, O> {
    pub fn append(&mut self, operator: O, right: ExprTree<T, O>) {
        match self {
            ExprTree::Value(_) | ExprTree::Enclosed(_) => unsafe {
                ptr::write(
                    self,
                    ExprTree::new_expression(ptr::read(self), operator, right),
                )
            },
            ExprTree::Expression {
                operator: self_operator,
                right: self_right,
                ..
            } => {
                if operator.precedence() > self_operator.precedence() {
                    self_right.append(operator, right);
                } else {
                    unsafe {
                        ptr::write(
                            self,
                            ExprTree::new_expression(ptr::read(self), operator, right),
                        )
                    }
                }
            }
        }
    }
}
