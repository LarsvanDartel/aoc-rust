pub use winnow::ascii::{
    alpha1, alphanumeric1, dec_int, dec_uint, hex_digit1, hex_uint, line_ending, multispace1,
    space0, space1,
};
pub use winnow::combinator::{
    alt, delimited, preceded, repeat, repeat_till, separated, separated_pair, terminated,
};
use winnow::error::ParserError;
pub use winnow::prelude::*;
use winnow::stream::{AsBStr, AsChar, Stream, StreamIsPartial};
pub use winnow::token::{any as anychar, one_of, take, take_till, take_until};

pub fn list<I, O, S, E, PN, PS>(parser: PN, separator: PS) -> impl Parser<I, Vec<O>, E>
where
    I: Stream,
    PN: Parser<I, O, E>,
    PS: Parser<I, S, E>,
    E: ParserError<I>,
{
    separated(1.., parser, separator)
}

pub fn many<I, O, E, PN>(parser: PN) -> impl Parser<I, Vec<O>, E>
where
    I: Stream,
    PN: Parser<I, O, E>,
    E: ParserError<I>,
{
    repeat(1.., parser)
}

pub fn dec_i32<I, E>(input: &mut I) -> PResult<i32, E>
where
    I: StreamIsPartial + Stream,
    <I as Stream>::Slice: AsBStr,
    <I as Stream>::Token: AsChar + Clone,
    E: ParserError<I>,
{
    dec_int(input)
}

pub fn dec_i64<I, E>(input: &mut I) -> PResult<i64, E>
where
    I: StreamIsPartial + Stream,
    <I as Stream>::Slice: AsBStr,
    <I as Stream>::Token: AsChar + Clone,
    E: ParserError<I>,
{
    dec_int(input)
}

pub fn dec_i128<I, E>(input: &mut I) -> PResult<i128, E>
where
    I: StreamIsPartial + Stream,
    <I as Stream>::Slice: AsBStr,
    <I as Stream>::Token: AsChar + Clone,
    E: ParserError<I>,
{
    dec_int(input)
}

pub fn dec_isize<I, E>(input: &mut I) -> PResult<isize, E>
where
    I: StreamIsPartial + Stream,
    <I as Stream>::Slice: AsBStr,
    <I as Stream>::Token: AsChar + Clone,
    E: ParserError<I>,
{
    dec_int(input)
}

pub fn dec_u32<I, E>(input: &mut I) -> PResult<u32, E>
where
    I: StreamIsPartial + Stream,
    <I as Stream>::Slice: AsBStr,
    <I as Stream>::Token: AsChar + Clone,
    E: ParserError<I>,
{
    dec_uint(input)
}

pub fn dec_u64<I, E>(input: &mut I) -> PResult<u64, E>
where
    I: StreamIsPartial + Stream,
    <I as Stream>::Slice: AsBStr,
    <I as Stream>::Token: AsChar + Clone,
    E: ParserError<I>,
{
    dec_uint(input)
}

pub fn dec_u128<I, E>(input: &mut I) -> PResult<u128, E>
where
    I: StreamIsPartial + Stream,
    <I as Stream>::Slice: AsBStr,
    <I as Stream>::Token: AsChar + Clone,
    E: ParserError<I>,
{
    dec_uint(input)
}

pub fn dec_usize<I, E>(input: &mut I) -> PResult<usize, E>
where
    I: StreamIsPartial + Stream,
    <I as Stream>::Slice: AsBStr,
    <I as Stream>::Token: AsChar + Clone,
    E: ParserError<I>,
{
    dec_uint(input)
}
