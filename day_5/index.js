import { PGlite } from "@electric-sql/pglite";
import { readFileSync } from "node:fs";
import { btree_gist } from "@electric-sql/pglite/contrib/btree_gist";

const db = new PGlite({
  extensions: { btree_gist },
});

console.time("total");
console.time("parsing");
const input = readFileSync("data/input.txt", "utf-8");
// const input = readFileSync("data/input_small.txt", "utf-8");
const [ranges, ids] = input.split("\n\n");

await db.exec(
  `
  CREATE TABLE IF NOT EXISTS range (
    range int8range
  );
  -- the index makes the join faster (16ms -> 13ms), but slows the where exists down (11ms -> 14ms)
  -- CREATE INDEX IF NOT EXISTS range_idx ON range USING GIST(range);
  CREATE TABLE IF NOT EXISTS range_staging (
    low INT8,
    high INT8
  );
  TRUNCATE TABLE range_staging;
  TRUNCATE TABLE range;
  COPY range_staging FROM '/dev/blob'
  (
    FORMAT text,
    DELIMITER '-',
    ON_ERROR ignore,
    FREEZE
    );
  INSERT INTO range
  SELECT int8range(stg.low, stg.high, '[]') from range_staging stg;
    `,
  { blob: new Blob([ranges]) }
);

await db.exec(
  `
  CREATE TABLE IF NOT EXISTS id (
    id INT8
  );
  TRUNCATE TABLE id;
  COPY id FROM '/dev/blob'
  (
    FORMAT text,
    ON_ERROR ignore,
    FREEZE
  );
`,
  { blob: new Blob([ids]) }
);

console.timeEnd("parsing");

console.time("star1");
const star1 = await db.query(`
  SELECT COUNT(*) FROM id i
  WHERE EXISTS (SELECT 1/0 FROM range r WHERE r.range @> i.id);
`);
console.timeEnd("star1");
console.log(star1.rows[0].count);
console.time("star1_2");
const star1_2 = await db.query(`
  SELECT COUNT(*) FROM (
    SELECT DISTINCT i.id FROM id i
    JOIN range r ON r.range @> i.id
  );
`);
console.timeEnd("star1_2");
console.log(star1_2.rows[0].count);

console.time("star2");
const star2 = await db.query(`
  WITH multi AS (
    SELECT range_agg(range) FROM range
  ),
  unnested AS (
    SELECT unnest(range_agg) FROM multi
  ),
  size AS (
    SELECT upper(unnest) - lower(unnest) AS size FROM unnested
  )
  SELECT sum(size) from size;
`);
console.timeEnd("star2");
console.log(star2.rows[0].sum);
console.timeEnd("total");
