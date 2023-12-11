use crate::solver::TwoPartsProblemSolver;
use crate::utils::int_range::IntRange;
use anyhow::{anyhow, bail, Context};
use std::borrow::Cow;

use derive_more::Display;
use num::PrimInt;
use regex::Regex;
use sscanf::sscanf;

use std::cmp::min;

use std::fmt::Debug;

use std::num::ParseIntError;

use std::str::FromStr;

pub struct Day5<T: PrimInt> {
    seeds: Vec<T>,
    data: Vec<(String, Vec<(IntRange<T>, IntRange<T>)>)>,
}

impl<T: PrimInt + FromStr<Err = ParseIntError> + Debug + Send + Sync + Display + 'static> FromStr
    for Day5<T>
{
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let double_newline_regex = Regex::new(r"\r?\n\r?\n")?;
        let mut parts = double_newline_regex.split(s);
        let seed_line = parts
            .next()
            .with_context(|| format!("No seed line found from input: {:?}", s))?;

        let seeds = sscanf!(seed_line, "seeds: {str}").map_err(|e| {
            anyhow!(
                "Cannot parse seed line for input: {:?}\nDescription: {}",
                seed_line,
                e
            )
        })?;
        let seeds = seeds
            .split_whitespace()
            .map(<T>::from_str)
            .collect::<Result<_, T::Err>>()?;

        let data: Vec<(String, Vec<(IntRange<T>, IntRange<T>)>)> = parts
            .map(|data_part| {
                let mut lines = data_part.lines();
                let map_line = lines
                    .next()
                    .with_context(|| format!("No map line found from input: {:?}", data_part))?;
                let map_name = sscanf!(map_line, "{str} map:").map_err(|e| {
                    anyhow!(
                        "Cannot parse map line for input: {:?}\nDescription: {}",
                        map_line,
                        e
                    )
                })?;
                let mut map_data = lines
                    .map(|line| {
                        line.split_whitespace().map(<T>::from_str).try_fold(
                            Vec::new(),
                            |mut l, r| {
                                if l.len() == 3 {
                                    bail!("Too many input in line {:?}", line);
                                };
                                l.push(r?);
                                Ok(l)
                            },
                        )
                    })
                    .map(|container_vec| {
                        container_vec.map(|v| {
                            Ok::<_, anyhow::Error>((
                                IntRange::new(v[1], v[1] + (v[2] - T::one()))?,
                                IntRange::new(v[0], v[0] + (v[2] - T::one()))?,
                            ))
                        })
                    })
                    .collect::<Result<Result<Vec<_>, _>, _>>()??;
                map_data.sort_unstable();
                return Ok::<_, anyhow::Error>((map_name.to_owned(), map_data));
            })
            .collect::<Result<_, _>>()?;
        return Ok(Day5 { seeds, data });
    }
}

impl<T> TwoPartsProblemSolver for Day5<T>
where
    T: PrimInt + Display + Debug + Send + Sync + FromStr<Err = ParseIntError> + 'static,
{
    type Target1 = T;
    type Target2 = T;

    fn solve_1(&self) -> anyhow::Result<T> {
        let mut seeds: Box<dyn Iterator<Item = T>> = Box::new(self.seeds.iter().map(T::clone));
        for (_, map) in &self.data {
            seeds = Box::new(seeds.map(move |s| get_from_range_to_range_maps(map, &s)))
        }
        return seeds.try_fold(T::max_value(), |a, b| Ok(min(a, b)));
    }

    fn solve_2(&self) -> anyhow::Result<T> {
        let seeds = self
            .seeds
            .chunks(2)
            .map(|v| IntRange::new(v[0], v[0] + (v[1] - T::one())))
            .collect::<Result<Vec<_>, _>>()?;

        return Ok(self
            .data
            .iter()
            .map(|(_, map)| map)
            .fold(Cow::from(seeds), |acc, maps| {
                Cow::from(get_range_from_range_to_range_maps(maps, acc.as_ref()))
            })
            .as_ref()
            .iter()
            .map(|i| i.start)
            .min()
            .unwrap());
    }
}

fn get_from_range_to_range_maps<
    'a,
    T: PrimInt + Debug + 'a,
    II: IntoIterator<Item = &'a (IntRange<T>, IntRange<T>)>,
>(
    range_to_range_maps: II,
    source: &T,
) -> T {
    for (source_map, dest_map) in range_to_range_maps {
        if let Some(value) = try_get_from_one_range_map(source_map, dest_map, source) {
            return value;
        }
    }
    return *source;
}

fn try_get_from_one_range_map<T: PrimInt + Debug>(
    source_map: &IntRange<T>,
    dest_map: &IntRange<T>,
    source: &T,
) -> Option<T> {
    if source_map.contains(source) {
        return Some(dest_map.start + (*source - source_map.start));
    }
    return None;
}

fn get_range_from_range_to_range_maps<'a, T, MI>(
    range_to_range_maps: MI,
    sources: &'a [IntRange<T>],
) -> Vec<IntRange<T>>
where
    T: PrimInt + Debug + 'a,
    MI: IntoIterator<Item = &'a (IntRange<T>, IntRange<T>)>,
{
    let (mut final_res, mut remainder) = range_to_range_maps.into_iter().fold(
        (Vec::new(), Cow::from(sources)),
        |(mut final_res, source), tuple_ref| {
            let (source_range, dest_range) = *tuple_ref;
            let source_ref = source.as_ref();
            let (mut res, remainder) =
                get_range_from_one_range_to_range_map(source_ref, &source_range, &dest_range);
            final_res.append(&mut res);
            return (final_res, Cow::from(remainder));
        },
    );

    final_res.append(&mut remainder.to_mut());
    return final_res;
}

fn get_range_from_one_range_to_range_map<'a, T, V>(
    sources: V,
    source_range: &IntRange<T>,
    dest_range: &IntRange<T>,
) -> (Vec<IntRange<T>>, Vec<IntRange<T>>)
where
    T: PrimInt + Debug + 'a,
    V: IntoIterator<Item = &'a IntRange<T>>,
{
    return sources
        .into_iter()
        .map(|source| (source.intersect(source_range), source.sub(source_range)))
        .fold(
            (Vec::new(), Vec::new()),
            |(mut res, mut remainder), (intersect_result, mut sub_result)| {
                if let Some(mut intersection) = intersect_result {
                    intersection -= source_range.start;
                    intersection += dest_range.start;
                    res.push(intersection);
                }
                remainder.append(&mut sub_result);
                return (res, remainder);
            },
        );
}

#[cfg(test)]
mod tests {
    use crate::solver::y2023::day5::Day5;
    use crate::solver::TwoPartsProblemSolver;

    use indoc::indoc;
    use std::str::FromStr;

    static SAMPLE_INPUT: &str = indoc! {"
            seeds: 79 14 55 13

            seed-to-soil map:
            50 98 2
            52 50 48

            soil-to-fertilizer map:
            0 15 37
            37 52 2
            39 0 15

            fertilizer-to-water map:
            49 53 8
            0 11 42
            42 0 7
            57 7 4

            water-to-light map:
            88 18 7
            18 25 70

            light-to-temperature map:
            45 77 23
            81 45 19
            68 64 13

            temperature-to-humidity map:
            0 69 1
            1 0 69

            humidity-to-location map:
            60 56 37
            56 93 4
    "};

    #[test]
    fn test_sample_1() -> anyhow::Result<()> {
        assert_eq!(Day5::<u32>::from_str(SAMPLE_INPUT)?.solve_1()?, 35);
        Ok(())
    }

    #[test]
    fn test_sample_2() -> anyhow::Result<()> {
        assert_eq!(Day5::<u32>::from_str(SAMPLE_INPUT)?.solve_2()?, 46);
        Ok(())
    }
}
