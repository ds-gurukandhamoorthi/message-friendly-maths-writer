use nom::{
    character::complete::{digit1, alpha1},
    bytes::complete::tag,
    sequence::separated_pair,
    branch::alt,
    IResult,
};

#[derive(Debug)]
pub enum Expression<'a>{
    Num(&'a str),
    Var(&'a str),
    Plus((Box<Expression<'a>>, Box<Expression<'a>>)),
    Multiply((Box<Expression<'a>>, Box<Expression<'a>>)),
    Power((Box<Expression<'a>>, Box<Expression<'a>>)),
}

fn parse_num(i: &str) -> IResult<&str, Expression> {
    let (i, expr) = digit1(i)?;
    Ok((i, Expression::Num(expr)))
}

fn parse_var(i: &str) -> IResult<&str, Expression> {
    let (i, expr) = alpha1(i)?;
    Ok((i, Expression::Var(expr)))
}

fn parse_atom(i: &str) -> IResult<&str, Expression> {
    alt((
            parse_num,
            parse_var,
    ))(i)
}


fn parse_power(i: &str) -> IResult<&str, Expression> {
    let (i, (base, exp)) = separated_pair(parse_atom, tag("^"), parse_expression)(i)?;
    Ok((i, Expression::Power((Box::new(base), Box::new(exp)))))
}

fn parse_plus(i: &str) -> IResult<&str, Expression> {
    let (i, (base, exp)) = separated_pair(parse_atom, tag("+"), parse_expression)(i)?;
    Ok((i, Expression::Plus((Box::new(base), Box::new(exp)))))
}

fn parse_multiply(i: &str) -> IResult<&str, Expression> {
    let (i, (base, exp)) = separated_pair(parse_atom, tag("*"), parse_expression)(i)?;
    Ok((i, Expression::Plus((Box::new(base), Box::new(exp)))))
}


pub fn parse_expression(i: &str) -> IResult<&str, Expression> {
    alt((
            parse_power,
            parse_multiply,
            parse_plus,
            parse_num,
            parse_var,
    ))(i)
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
