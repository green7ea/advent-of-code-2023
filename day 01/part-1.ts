import _ from "npm:lodash";
import { assertEquals } from "https://deno.land/std@0.208.0/assert/mod.ts";

import input from "./input.ts";

const sample = [
  ["1abc2", 12],
  ["pqr3stu8vwx", 38],
  ["a1b2c3d4e5f", 15],
  ["treb7uchet", 77],
];

Deno.test("Validate line by line", () => {
  const calculated = _(sample)
    .map(([str]: [string]) => [str, extract_digits(str)])
    .value();

  assertEquals(sample, calculated);
});

Deno.test("Validate sum", () => {
  const sample_text = _(sample).map(_.first).join("\n");
  assertEquals(sum_lines(sample_text), 142);
});

/**
 * Extracts the first and last digit found in a string. The first
 * digit and the last can be the same character if there is only one
 * digit in the string.
 */
function extract_digits(line: string): number {
  const digits = _(line)
    .split("")
    .filter((c: string) => /^\d$/.test(c))
    .value();

  const first = _.first(digits);
  const last = _.last(digits);
  const result = _([first, last]).join("");

  return Number.parseInt(result);
}

/**
 * Takes an advent day 1 input, splits it into lines, extracts number
 * from the first and last digits and sums these numbers to get a
 * result.
 */
function sum_lines(full_input: string): number {
  return _(full_input)
    .split("\n")
    .map(extract_digits)
    .reduce((a: number, b: number) => a + b);
}

console.log(sum_lines(input));
