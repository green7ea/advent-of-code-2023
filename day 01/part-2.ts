import _ from "npm:lodash";
import { assertEquals } from "https://deno.land/std@0.208.0/assert/mod.ts";

import input from "./input.ts";

const sample = [
  ["two1nine", 29],
  ["eightwothree", 83],
  ["abcone2threexyz", 13],
  ["xtwone3four", 24],
  ["4nineeightseven2", 42],
  ["zoneight234", 14],
  ["7pqrstsixteen", 76],
];

Deno.test("Validate line by line", () => {
  const calculated = _(sample)
    .map(([str]: [string]) => [str, extract_digits(str)])
    .value();

  assertEquals(sample, calculated);
});

Deno.test("Validate sum", () => {
  const sample_text = _(sample).map(_.first).join("\n");
  assertEquals(281, sum_lines(sample_text));
});

Deno.test("Tricky one", () => {
  const tricky_text = "sevenine";
  assertEquals(79, extract_digits(tricky_text));
});

Deno.test("Extract digits", () => {
  assertEquals("4", to_digit("four"));
  assertEquals("5", to_digit("5"));
});

/**
 * Converts a word that reprensents a digit, 'five', or word that is a
 * digit, '5', to a word that is a digit '5'.
 */
function to_digit(word: string): string | undefined {
  const lookup = {
    zero: "0",
    one: "1",
    two: "2",
    three: "3",
    four: "4",
    five: "5",
    six: "6",
    seven: "7",
    eight: "8",
    nine: "9",
  };

  if (/^\d$/.test(word)) {
    return word;
  }

  return _.get(lookup, word);
}

/**
 * This function is similar to javascript's built-in match but it
 * finds overlapping results as well. For example, if we're looking
 * for 'seven' or 'nine' and have the string 'sevenine', we would find
 * ['seven', 'nine'] and not just ['seven'] like the built-in does.
 */
function get_matches(regex: RegExp, str: string): string[] {
  let m;
  const res = [];

  while (m = regex.exec(str)) {
    res.push(_(m).filter((x: any) => !!x).first());
    regex.lastIndex = m.index + 1;
  }

  return res;
}

/**
 * Takes a line that follows the example's pattern and extracts the
 * first and last digit, returning those as a concatenated number.
 */
function extract_digits(line: string): number {
  const regex =
    /(?=(\d)|(zero)|(one)|(two)|(three)|(four)|(five)|(six)|(seven)|(eight)|(nine))/g;
  const matches = get_matches(regex, line);
  const digits = _.map(matches, to_digit);

  const first = _.first(digits);
  const last = _.last(digits);
  const result = _([first, last]).join("");

  return Number.parseInt(result);
}

/**
 * Splits the lines, extracts a number from their digits and computes
 * the sum.
 */
function sum_lines(full_input: string): number {
  return _(full_input)
    .split("\n")
    .map(extract_digits)
    .reduce((a: number, b: number) => a + b);
}

console.log(sum_lines(input));
