import { PGlite } from "@electric-sql/pglite";
import { readFileSync } from "node:fs";

const db = new PGlite();

const input = readFileSync("data/input.txt", "utf-8");
// const input = readFileSync("data/input_small.txt", "utf-8");
const [ranges, ids] = input.split("\n\n");

await db.exec(
  `
  CREATE TABLE IF NOT EXISTS range (
    range int8range
  );
` +
    ranges
      .split("\n")
      .map((range) => {
        const [low, high] = range.split("-");
        return `INSERT INTO range VALUES ('[${low}, ${high}]'::int8range);`;
      })
      .join("\n")
);

await db.exec(
  `
  CREATE TABLE IF NOT EXISTS id (
    id INT8
  );
` +
    ids
      .split("\n")
      .filter(Boolean)
      .map((id) => `INSERT INTO id VALUES (${id});`)
      .join("\n")
);

const star1 = await db.query(`
  SELECT COUNT(*) FROM id i
  WHERE EXISTS (SELECT 1/0 FROM range r WHERE r.range @> i.id);
`);
console.log(star1.rows[0].count);

const star2 = await db.query(`
  WITH multi AS (
    SELECT range_agg(range) FROM (SELECT range FROM range)
  ),
  unnested AS (
    SELECT unnest(range_agg) FROM multi
  ),
  size AS (
    SELECT upper(unnest) - lower(unnest) AS size FROM unnested
  )
  SELECT sum(size) from size;
`);
console.log(star2.rows);
