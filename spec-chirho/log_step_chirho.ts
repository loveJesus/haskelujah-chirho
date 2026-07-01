// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

import { Database } from "bun:sqlite";

const dbPathChirho = new URL("./progress-chirho.sqlite", import.meta.url).pathname;

function argValueChirho(nameChirho: string, fallbackChirho = ""): string {
  const flagChirho = `--${nameChirho}`;
  const idxChirho = process.argv.indexOf(flagChirho);
  if (idxChirho === -1) {
    return fallbackChirho;
  }
  return process.argv[idxChirho + 1] ?? fallbackChirho;
}

const agentCodeChirho = argValueChirho("agent-code-chirho", "gpt_chirho");
const actionTakenChirho = argValueChirho("action-taken-chirho");
const resultOfActionChirho = argValueChirho("result-of-action-chirho");
const overviewOfResultChirho = argValueChirho("overview-of-result-chirho", "went-as-planned");
const timestampStartChirho = argValueChirho(
  "timestamp-start-chirho",
  new Date().toISOString(),
);
const timestampEndChirho = argValueChirho(
  "timestamp-end-chirho",
  new Date().toISOString(),
);

if (!actionTakenChirho) {
  console.error("missing --action-taken-chirho");
  process.exit(2);
}

const dbChirho = new Database(dbPathChirho);
dbChirho.run(`
  CREATE TABLE IF NOT EXISTS steps_taken_chirho (
    id_chirho INTEGER PRIMARY KEY AUTOINCREMENT,
    agent_code_chirho TEXT NOT NULL,
    timestamp_start_chirho TEXT NOT NULL,
    timestamp_end_chirho TEXT,
    action_taken_chirho TEXT NOT NULL,
    result_of_action_chirho TEXT,
    overview_of_result_chirho TEXT
  )
`);

dbChirho
  .query(
    `
      INSERT INTO steps_taken_chirho (
        agent_code_chirho,
        timestamp_start_chirho,
        timestamp_end_chirho,
        action_taken_chirho,
        result_of_action_chirho,
        overview_of_result_chirho
      )
      VALUES (?, ?, ?, ?, ?, ?)
    `,
  )
  .run(
    agentCodeChirho,
    timestampStartChirho,
    timestampEndChirho,
    actionTakenChirho,
    resultOfActionChirho,
    overviewOfResultChirho,
  );
