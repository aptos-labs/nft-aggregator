import { getPostgresClient } from "../lib/utils";

const run = async () => {
  const result = await getPostgresClient()(`SELECT * FROM processor_status`);
  console.log(result);
};

run();
