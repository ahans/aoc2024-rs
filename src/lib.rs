#![feature(portable_simd)]
#![feature(avx512_target_feature)]
#![feature(adt_const_params)]

extern crate aoc_runner;

#[macro_use]
extern crate aoc_runner_derive;

pub mod day1;
pub mod day10;
pub mod day11;
pub mod day13;
pub mod day14;
pub mod day2;
pub mod day21;
pub mod day22;
pub mod day25;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;

aoc_lib! { year = 2024 }
