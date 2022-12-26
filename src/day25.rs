use std::{iter::Sum, ops::Add, str::FromStr};

use num::Zero;

/*
Day 25 introduces a novel numbering system. Instead of normal base-10 numbers,
it uses a base-5 system - but more than that, it's a *balanced* base-5 system,
using the symbols '=' (-2), '-' (-1), 0, 1, and 2.

Getting the first star requires you to read in a list of numbers written using
this system, sum them up, and write out the result in balanced quinary.

The obvious (and straightforward) way to approach this is to convert each string
of symbols into a native-format integer, add those integers, and convert back
to balanced quinary for output.

I chose instead to build a (limited) implementation of balanced quinary from scratch,
thereby avoiding any pesky conversions to other bases and keeping the computations
pure and simple (?).
*/

/**
 * Represents a single symbol (digit-equivalent) for balanced quinary.
 */
#[derive(Clone, Copy, PartialEq)]
enum Quint {
    MinusTwo,
    MinusOne,
    Zero,
    One,
    Two,
}

impl TryFrom<char> for Quint {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '=' => Ok(Self::MinusTwo),
            '-' => Ok(Self::MinusOne),
            '0' => Ok(Self::Zero),
            '1' => Ok(Self::One),
            '2' => Ok(Self::Two),
            _ => Err("invalid char"),
        }
    }
}

impl From<Quint> for char {
    fn from(pent: Quint) -> Self {
        match pent {
            Quint::MinusTwo => '=',
            Quint::MinusOne => '-',
            Quint::Zero => '0',
            Quint::One => '1',
            Quint::Two => '2',
        }
    }
}

/**
 * Implements a half-adder for Quints.
 *
 * `lhs_quint + rhs_quint` returns a pair (sum, carry).
 *
 * Unfortunately, since we're working entirely symbolically, the best
 * we can do here is write down the addition table as concisely as possible.
 */
impl Add for Quint {
    type Output = (Self, Self);

    fn add(self, other: Self) -> (Self, Self) {
        match (self, other) {
            // 0 + X = X
            (Self::Zero, any) | (any, Self::Zero) => (any, Self::Zero),

            // X + -X = 0
            (Self::One, Self::MinusOne)
            | (Self::Two, Self::MinusTwo)
            | (Self::MinusOne, Self::One)
            | (Self::MinusTwo, Self::Two) => (Self::Zero, Self::Zero),

            // There are only a couple of ways to get -1 or 1 with no carry
            (Self::Two, Self::MinusOne) | (Self::MinusOne, Self::Two) => (Self::One, Self::Zero),
            (Self::MinusTwo, Self::One) | (Self::One, Self::MinusTwo) => {
                (Self::MinusOne, Self::Zero)
            }

            // There's only one way each to get 2 or -2 with no carry
            (Self::One, Self::One) => (Self::Two, Self::Zero),
            (Self::MinusOne, Self::MinusOne) => (Self::MinusTwo, Self::Zero),

            // Finally we get to the operands that result in carries.
            // For example, 2 + 1 = 3 = (-2) + (1)*5, so the sum is -2 and the carry is 1.
            (Self::Two, Self::One) | (Self::One, Self::Two) => (Self::MinusTwo, Self::One),
            (Self::MinusTwo, Self::MinusOne) | (Self::MinusOne, Self::MinusTwo) => {
                (Self::Two, Self::MinusOne)
            }

            // e.g. 2 + 2 = 4 = (-1) + (1)*5
            (Self::Two, Self::Two) => (Self::MinusOne, Self::One),
            (Self::MinusTwo, Self::MinusTwo) => (Self::One, Self::MinusOne),
        }
    }
}

/**
 * Represents an arbitrary integer as a string of quints.
 *
 * Note that quints are stored in little-endian order, with the least-
 * significant quint first. This simplifies operations and makes them
 * a bit faster.
 */
#[derive(PartialEq)]
struct BalancedQuinary {
    quints: Vec<Quint>,
}

impl FromStr for BalancedQuinary {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.chars()
            .rev()
            .map(|c| c.try_into())
            .collect::<Result<_, _>>()
            .map(|quints| BalancedQuinary { quints })
    }
}

impl From<BalancedQuinary> for String {
    fn from(value: BalancedQuinary) -> Self {
        value
            .quints
            .iter()
            .rev()
            .map(|&quint| char::from(quint))
            .collect::<String>()
    }
}

impl Zero for BalancedQuinary {
    fn zero() -> Self {
        Self {
            quints: vec![Quint::Zero],
        }
    }

    fn is_zero(&self) -> bool {
        self.quints.is_empty() || (self.quints.len() == 1 && matches!(self.quints[0], Quint::Zero))
    }
}

/**
 * The only real operation implemented for balanced quinary: addition.
 *
 * Effectively, this builds an awkward full adder out of the half-adder
 * implemented in Quint::add.
 */
impl Add for BalancedQuinary {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut quints = Vec::new();
        let mut carry = Quint::Zero;

        let shorter_len = self.quints.len().min(rhs.quints.len());
        for i in 0..shorter_len {
            // Add the current quints.
            let (quint_sum, generated_carry) = self.quints[i] + rhs.quints[i];

            // Add the input carry to the sum.
            let (sum, propagated_carry) = quint_sum + carry;

            // Add the carries. The carry from this sum can *never* be nonzero, since
            // the only possible inputs are One and MinusOne, and no addition involving
            // only those values can result in a nonzero carry.
            let (total_carry, _) = generated_carry + propagated_carry;

            quints.push(sum);
            carry = total_carry;
        }

        // We reached the end of the smaller number's quints; the larger number
        // may have more quints to add in. For each of those, propagate the carry
        // through.
        // Note that at least one of these loops will do nothing.

        for i in shorter_len..self.quints.len() {
            let (sum, new_carry) = carry + self.quints[i];
            quints.push(sum);
            carry = new_carry;
        }

        for i in shorter_len..rhs.quints.len() {
            let (sum, new_carry) = carry + rhs.quints[i];
            quints.push(sum);
            carry = new_carry;
        }

        // If there is a carry left over at this point, we need to add it
        // as the highest-order quint of the result.
        if !matches!(carry, Quint::Zero) {
            quints.push(carry);
        }

        // Let's establish a convention that every number has at least one quint.
        // This avoids awkward empty strings when printing, for example.
        if quints.is_empty() {
            quints.push(Quint::Zero);
        }

        Self { quints }
    }
}

/**
 * Convenient trait so we can call .sum() on iterators of balanced quinary numbers.
 */
impl Sum for BalancedQuinary {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(BalancedQuinary::zero(), |acc, n| acc + n)
    }
}

#[aoc(day25, part1)]
pub fn part1(input: &str) -> String {
    let total: BalancedQuinary = input
        .lines()
        .filter_map(|line| BalancedQuinary::from_str(line).ok())
        .sum();
    String::from(total)
}

#[cfg(test)]
mod tests {
    use super::part1;

    const EXAMPLE: &str = "1=-0-2\n\
                           12111\n\
                           2=0=\n\
                           21\n\
                           2=01\n\
                           111\n\
                           20012\n\
                           112\n\
                           1=-1=\n\
                           1-12\n\
                           12\n\
                           1=\n\
                           122\n";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&EXAMPLE), "2=-1=0");
    }
}
